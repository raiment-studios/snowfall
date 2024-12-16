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

    let mut dirt_block = rng.select_fn(vec!["dirt", "dirt2"]);
    let mut grass_block = rng.select_fn(vec!["grass1", "grass2"]);

    for y in -R..=R {
        for x in -R..=R {
            let u0 = (x as f32) / (R as f32);
            let v0 = (y as f32) / (R as f32);

            let jitter_radius = 1.5 * noise4.gen_2d(u0, v0);
            let jitter_angle = 2.0 * PI * noise3.gen_2d(u0, v0);
            let h3 = 64.0 * jitter_radius * (0.5 + 0.5 * jitter_angle.cos());
            let h = h3.powf(1.45);

            // Smooth influnece to 0 around the tile edges
            let base_z = ctx.ground_height_at(x, y).unwrap_or(1);

            // Draw the voxels
            for z in 0..=(h.round() as i32) {
                let block = grass_block();
                model.set_voxel((x, y, base_z + z), block);
            }
        }
    }

    let mut goal: (i32, i32, i32) = (rng.range((R / 2)..R), rng.range((R / 2)..=R), 0);
    goal.2 = model.height_at(goal.0, goal.1).unwrap_or(0);
    let mut start: (i32, i32, i32) = (rng.range(-R..=(-R / 2)), rng.range(-R..=(-R / 2)), 0);
    start.2 = model.height_at(start.0, start.1).unwrap_or(0);

    let mut cache: HashMap<(i32, i32), i32> = HashMap::new();

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
        |&(x, y, z)| 35 * (goal.0.abs_diff(x) + goal.1.abs_diff(y) + goal.2.abs_diff(z)),
        |&p| p == goal,
    );

    if let Some((path, _cost)) = result {
        for (x, y, z) in path {
            for dz in 0..=10 {
                //model.set_voxel((x, y, z + dz), "red");
            }
            let bz = *cache
                .entry((x, y))
                .or_insert_with(|| model.height_at(x, y).unwrap_or(0));
            for dx in -2..=2 {
                for dy in -2..=2 {
                    let nx = x + dx;
                    let ny = y + dy;
                    let nz: i32 = *cache
                        .entry((nx, ny))
                        .or_insert_with(|| model.height_at(nx, ny).unwrap_or(0));

                    for z in nz..=bz {
                        model.set_voxel((nx, ny, z), "empty");
                    }
                    model.set_voxel((nx, ny, bz), dirt_block());
                }
            }
        }
    }

    model
}

fn rotate_2d(u: f32, v: f32, angle: f32) -> (f32, f32) {
    let u2 = u * angle.cos() - v * angle.sin();
    let v2 = u * angle.sin() + v * angle.cos();
    (u2, v2)
}
