use crate::internal::*;

pub fn hill_with_road(ctx: &GenContext, scene: &mut Scene2) -> Group {
    let mut rng = ctx.make_rng();

    scene.terrain = generators::hill4(&ctx.fork("hill", rng.seed8()), scene);
    for _ in 0..4 {
        road(&ctx.fork("road", rng.seed8()), scene);
    }

    let mut group = Group::new();
    for _ in 0..4 {
        let seed = rng.seed8();

        let mut ctx = ctx.fork("cluster2", seed);
        //ctx.center = IVec3::new(rng.range(-200..=200), rng.range(-200..=200), 0);
        ctx.params = serde_json::json!({
            "count": [320, 400],
            "range": 248,
        });
        let g: VoxelModel = generate_model(&ctx, scene);
        let VoxelModel::Group(g) = g else {
            panic!("expected group");
        };
        for object in g.objects {
            group.objects.push(object);
        }
    }
    group
}

fn road(ctx: &GenContext, scene: &mut Scene2) {
    let mut rng = ctx.make_rng();
    for _ in 0..4 {
        let ctx = ctx.fork("road", rng.seed8());
        if road_imp(&ctx, scene).is_ok() {
            return;
        }
    }
}

fn road_imp(ctx: &GenContext, scene: &mut Scene2) -> Result<(), String> {
    use std::f32::consts::PI;

    let mut rng = ctx.make_rng();
    let model = &mut scene.terrain;

    model.register_block(Block::color("road1", 25, 20, 10).modify(|b| b.walk_cost = 0.15));
    model.register_block(Block::color("road2", 20, 15, 10).modify(|b| b.walk_cost = 0.15));

    let mut road_block = rng.select_fn(vec!["road1", "road2"]);

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
    let mut iterations = 0;
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

            const MAX_COST: f32 = 25_000.0;
            fn cost_as_u32(cost: f32) -> u32 {
                (cost.min(MAX_COST) * 1000.0).round() as u32
            }

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

                    let block = model.get_voxel((new_x, new_y, new_z));
                    let walk_cost = block.walk_cost;

                    let distance_xy: f32 = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();
                    let dz_factor: f32 = if dz < -8 {
                        100.0
                    } else if dz < -4 {
                        2.0
                    } else if dz < -2 {
                        1.5
                    } else if dz < 0 {
                        0.75
                    } else if dz == 0 {
                        1.0
                    } else if dz < 2 {
                        1.25
                    } else if dz < 4 {
                        1.5
                    } else if dz < 8 {
                        2.0
                    } else if dz < 16 {
                        5.0
                    } else {
                        100.0
                    };

                    let cost = distance_xy * dz_factor * (1.0 + 10.0 * walk_cost);
                    let cost = cost_as_u32(cost);
                    Some(((new_x, new_y, new_z), cost))
                })
                .collect::<Vec<_>>();

            iterations += 1;
            if iterations > 1_000_000 {
                panic!("too many iterations");
            }

            costs.into_iter()
        },
        |&(x, y, z)| {
            let d = end.0.abs_diff(x) + end.1.abs_diff(y) + end.2.abs_diff(z);
            d * 1000 * 100 * 10
        },
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
        const R2: i32 = R + 4;

        let line = bresenham3d(p, q);
        for IVec3 { x, y, z } in &line {
            for dx in -R..=R {
                for dy in -R..=R {
                    for dz in 1..=12 {
                        model.clear_voxel((x + dx, y + dy, z + dz));
                    }
                }
            }
        }
        for IVec3 { x, y, z } in &line {
            for dx in -R..=R {
                for dy in -R..=R {
                    let z = model.height_at(x + dx, y + dy).unwrap_or(0);
                    let p = (x + dx, y + dy, z);
                    let block = road_block();
                    model.set_voxel(p, block);
                }
            }
        }

        // Mark the road and area around it as "occupied" so other objects
        // are not placed on top of it.
        for IVec3 { x, y, z } in &line {
            for dx in -R2..=R2 {
                for dy in -R2..=R2 {
                    let z = model.height_at(x + dx, y + dy).unwrap_or(0);
                    model.modify_voxel((x + dx, y + dy, z), |block| block.with_occupied(true));
                }
            }
        }
    }
    Ok(())
}
