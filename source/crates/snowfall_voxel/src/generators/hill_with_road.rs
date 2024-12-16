use pathfinding::matrix::directions::N;

use crate::internal::*;

pub fn hill_with_road(seed: u64, ctx: &GenContext) -> VoxelSet {
    let mut rng = RNG::new(seed);

    let mut model = VoxelSet::new();
    model.register_block(Block::color("dirt", 25, 20, 10));
    model.register_block(Block::color("dirt2", 20, 15, 10));
    model.register_block(Block::color("grass1", 5, 60, 10));
    model.register_block(Block::color("grass2", 3, 45, 2));
    model.register_block(Block::color("red", 255, 0, 0));
    model.register_block(Block::color("blue", 0, 0, 255));

    use std::f32::consts::PI;
    const R: i32 = 256;
    let noise4 = rng.open_simplex().scale(0.25).build();
    let noise3 = rng.open_simplex().scale(0.5).build();

    let mut grass_block = rng.select_fn(vec!["grass1", "grass2"]);

    for y in -R..=R {
        for x in -R..=R {
            let u0 = (x as f32) / (R as f32);
            let v0 = (y as f32) / (R as f32);

            let jitter_radius = 1.5 * noise4.gen_2d(u0, v0);
            let jitter_angle = 2.0 * PI * noise3.gen_2d(u0, v0);
            let h3 = 64.0 * jitter_radius * (0.5 + 0.5 * jitter_angle.cos());
            let h = h3.powf(1.15);

            // Smooth influnece to 0 around the tile edges
            let base_z = ctx.ground_height_at(x, y).unwrap_or(1);

            // Draw the voxels
            for z in 0..=(h.round() as i32) {
                let block = grass_block();
                model.set_voxel((x, y, base_z + z), block);
            }
        }
    }

    for _ in 0..4 {
        if road(rng.range(1..=8192), &mut model).is_ok() {
            break;
        }
    }

    model
}

fn road(seed: u64, model: &mut VoxelSet) -> Result<(), String> {
    let mut rng = RNG::new(seed);
    let mut dirt_block = rng.select_fn(vec!["dirt", "dirt2"]);

    //
    // Choose the start and end points of the road segment.
    //
    // Ensure they are in different quadrants of the rectangular
    // area and are not too close to the center.
    //
    let quadrants = vec![(1, 1), (-1, 1), (-1, -1), (1, -1)];
    let selected = rng.select_n(2, &quadrants);

    let mut gen_range = || rng.range(200..=250);
    let mut goal: (i32, i32, i32) = (gen_range(), gen_range(), 0);
    goal.0 *= selected[0].0;
    goal.1 *= selected[0].1;
    goal.2 = model.height_at(goal.0, goal.1).unwrap_or(0);
    let mut start: (i32, i32, i32) = (gen_range(), gen_range(), 0);
    start.0 *= selected[1].0;
    start.1 *= selected[1].1;
    start.2 = model.height_at(start.0, start.1).unwrap_or(0);

    //
    // Cache the height look-ups since there are many look-ups
    //
    let mut cache: HashMap<(i32, i32), i32> = HashMap::new();

    // Call the A* pathfinding algorithm to generate the connection
    // from start to end.
    //
    // Be wary:
    // - If no path exists, astar() never returns (!)
    // - If the heuristic is *less expensive* than the actual
    //   lowest cost path, then the algorithm breaks
    //
    let mut count = 0;
    let result = pathfinding::prelude::astar(
        &start,
        |&(x, y, z)| {
            let moves = vec![
                (1, 0), //
                (-1, 0),
                (0, 1),
                (0, -1),
                (1, 1),
                (-1, 1),
                (1, -1),
                (-1, -1),
            ];

            let costs = moves
                .iter()
                .filter_map(|&(dx, dy)| {
                    let new_x = x + dx;
                    let new_y = y + dy;
                    let z: i32 = *cache
                        .entry((x, y))
                        .or_insert_with(|| model.height_at(x, y).unwrap_or(0));
                    let new_z: i32 = *cache
                        .entry((new_x, new_y))
                        .or_insert_with(|| model.height_at(new_x, new_y).unwrap_or(0));
                    let dz = new_z - z;
                    let cost = 2 * match dz {
                        -2 => 10,
                        -1 => 1,
                        0 => 2,
                        1 => 10,
                        2 => 30,
                        _ => 100,
                    } + if dx.abs() + dy.abs() > 1 { 3 } else { 2 };
                    Some(((new_x, new_y, new_z), cost))
                })
                .collect::<Vec<_>>();

            count += 1;
            if count > 1_000_000 {
                panic!("too many iterations");
            }

            costs.into_iter()
        },
        |&(x, y, z)| 100 * (goal.0.abs_diff(x) + goal.1.abs_diff(y) + goal.2.abs_diff(z)),
        |&p| p == goal,
    );

    //
    // Segmentize.
    //
    // Given the found path, break it into a series of straight line segments
    // which "feels" more natural of a constructed road than exact path finding
    //
    let path = match result {
        Some((path, _cost)) => path,
        None => {
            return Err("No path found".to_string());
        }
    };

    let mut posts = Vec::new();
    for i in (0..path.len() - 6).step_by(12) {
        posts.push(path[i]);
    }
    posts.push(path[path.len() - 1]);

    //
    // Connect the segments.
    //
    // Do this by flattening the terrain in a brief radius around the
    // path and painting the ground voxels to the road color.
    //
    for pair in posts.windows(2) {
        let p = IVec3::new(pair[0].0, pair[0].1, pair[0].2);
        let q = IVec3::new(pair[1].0, pair[1].1, pair[1].2);

        const R: i32 = 3;

        let line = bresenham3d(p, q);
        for IVec3 { x, y, z } in &line {
            for dx in -R..=R {
                for dy in -R..=R {
                    for dz in 1..=12 {
                        model.set_voxel((x + dx, y + dy, z + dz), "empty");
                    }
                }
            }
        }
        for IVec3 { x, y, z } in line {
            for dx in -R..=R {
                for dy in -R..=R {
                    let block = dirt_block();
                    model.set_voxel((x + dx, y + dy, z), block);
                }
            }
        }
    }
    Ok(())
}

fn rotate_2d(u: f32, v: f32, angle: f32) -> (f32, f32) {
    let u2 = u * angle.cos() - v * angle.sin();
    let v2 = u * angle.sin() + v * angle.cos();
    (u2, v2)
}
