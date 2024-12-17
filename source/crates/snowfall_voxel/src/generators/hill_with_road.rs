use crate::{internal::*, voxel_set};

pub fn hill_with_road(ctx: &GenContext, scene: &mut Scene2) -> Group {
    let mut rng = ctx.make_rng();

    scene.terrain = hill(&ctx.fork("hill", rng.seed8k()), scene);
    road(&ctx.fork("road", rng.seed8k()), scene);

    let mut group = Group::new();
    for _ in 0..4 {
        let seed = rng.seed8k();
        let mut ctx = ctx.fork("cluster", seed);
        ctx.center = IVec3::new(rng.range(-200..=200), rng.range(-200..=200), 0);
        ctx.params = serde_json::json!({
            "count": [12, 24],
            "range": 48,
        });
        let g = cluster2(&ctx, scene);
        for object in g.objects {
            group.objects.push(object);
        }
    }
    group
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct ClusterParams {
    count: Option<[i64; 2]>,
    range: Option<i32>,
}

pub fn cluster2(ctx: &GenContext, scene: &Scene2) -> Group {
    let mut rng = ctx.make_rng();

    let mut params: ClusterParams = ctx.params();
    let count_range = params.count.get_or_insert([12, 24]);
    let range = *params.range.get_or_insert(48);

    const MAX_ATTEMPTS: usize = 128;
    const CLOSEST_DISTANCE: f32 = 12.0;
    let mut count = rng.range(count_range[0]..=count_range[1]);

    let mut group = Group::new();

    let mut point_set = PointSet::new();
    for _ in 0..MAX_ATTEMPTS {
        let model_id = *rng.select_weighted(&vec![
            (10, "tree1"), //
            (10, "tree2"),
            (80, "pine_tree"),
        ]);
        let position = IVec3::new(rng.range(-range..=range), rng.range(-range..=range), 0);
        let position = position + ctx.center;

        let d = point_set.nearest_distance(&position).unwrap_or(f32::MAX);
        if d < CLOSEST_DISTANCE {
            continue;
        }

        point_set.add(position);

        let seed = rng.seed8k();
        let voxel_set: VoxelSet = match model_id {
            "tree1" => generators::tree1(&ctx.fork("tree1", seed), scene),
            "tree2" => generators::tree2(&ctx.fork("tree2", seed), scene),
            "pine_tree" => generators::pine_tree(&ctx.fork("pine_tree", seed), scene),
            _ => panic!("unknown model_id"),
        };

        group.objects.push(Object {
            generator_id: model_id.to_string(),
            seed,
            params: serde_json::Value::Null,
            position,
            imp: ObjectImp::VoxelSet(Box::new(voxel_set)),
        });

        count -= 1;
        if count == 0 {
            break;
        }
    }
    group
}

fn hill(ctx: &GenContext, scene: &mut Scene2) -> VoxelSet {
    let mut rng = ctx.make_rng();

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

            let base_z = scene.terrain.height_at(x, y).unwrap_or(1);

            // Draw the voxels
            for z in 0..=(h.round() as i32) {
                let block = grass_block();
                model.set_voxel((x, y, base_z + z), block);
            }
        }
    }

    model
}

fn road(ctx: &GenContext, scene: &mut Scene2) {
    let mut rng = ctx.make_rng();
    for _ in 0..4 {
        let ctx = ctx.fork("road", rng.seed8k());
        if road_imp(&ctx, scene).is_ok() {
            return;
        }
    }
}

fn road_imp(ctx: &GenContext, scene: &mut Scene2) -> Result<(), String> {
    use std::f32::consts::PI;

    let mut rng = ctx.make_rng();
    let model = &mut scene.terrain;
    let mut dirt_block = rng.select_fn(vec!["dirt", "dirt2"]);

    //
    // Choose the start and end points of the road segment.
    //
    let start_radius = rng.range(200.0..=250.0);
    let start_ang = rng.radians();
    let end_radius = rng.range(200.0..=250.0);
    let end_ang = start_ang + PI + rng.range(-PI / 10.0..PI / 10.0);
    let start = (
        (start_radius * start_ang.cos()).round() as i32,
        (start_radius * start_ang.sin()).round() as i32,
        0 as i32,
    );
    let end = (
        (end_radius * end_ang.cos()).round() as i32,
        (end_radius * end_ang.sin()).round() as i32,
        0 as i32,
    );

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
        |&(x, y, _z)| {
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
        |&(x, y, z)| 100 * (end.0.abs_diff(x) + end.1.abs_diff(y) + end.2.abs_diff(z)),
        |&p| p.0 == end.0 && p.1 == end.1,
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
