use crate::internal::*;

pub fn maple(ctx: &GenContext, scene: &mut Scene2) -> Group {
    use std::f32::consts::PI;

    let mut rng = ctx.make_rng();

    let mut group = Group::new();

    for _ in 0..8 {
        let angle = rng.range(0.0..=2.0 * PI);
        let radius = 1.25 * rng.range(16.0..=48.0);
        let x = radius * angle.cos();
        let y = radius * angle.sin();
        let base = IVec3::new(x as i32, y as i32, 0);

        let sub = ctx.fork("trunk", rng.seed8());
        let trunk = bare_tree(&sub, scene);

        group.objects.push(Object {
            generator_id: "trunk".to_string(),
            seed: 0,
            position: base,
            params: serde_json::Value::Null,
            imp: ObjectImp::VoxelSet(Box::new(trunk)),
        });
    }

    group
}

pub fn bare_tree(ctx: &GenContext, _scene: &Scene2) -> VoxelSet {
    let mut rng = ctx.make_rng();

    let mut trunk = VoxelSet::new();
    trunk.register_block(Block::color("brown1", 60, 40, 20));
    trunk.register_block(Block::color("brown2", 30, 20, 5));
    trunk.register_block(Block::color("brown3", 22, 15, 4));
    trunk.register_block(Block::color("c1", 255, 0, 0));
    trunk.register_block(Block::color("c2", 0, 255, 0));
    trunk.register_block(Block::color("c3", 0, 0, 255));
    trunk.register_block(Block::color("c4", 255, 255, 0));
    trunk.register_block(Block::color("c5", 255, 0, 255));
    trunk.register_block(Block::color("c6", 0, 255, 255));

    let mut base_points = vec![(IVec3::ZERO.clone(), 10)];
    let mut count = 0;
    let mut segment = 0;
    while !base_points.is_empty() {
        let set = base_points.drain(..).collect::<Vec<_>>();
        for (p, h) in set {
            let mut branches = Vec::new();

            let len = h - rng.range(1..=2);
            if len <= 2 {
                continue;
            }

            let bcount = match segment {
                0 => 1,
                1 => 2,
                _ => rng.range(1..=(1 + segment)),
            };
            for _ in 0..bcount {
                let dirs = vec![(1, 0), (0, 1), (1, 1)];
                let mut dir = *rng.select(&dirs);
                let sign = *rng.select(&vec![-1, 1]);
                let scale = rng.range(1..=(1 + segment * 2).min(len + 1));
                dir.0 *= scale * sign;
                dir.1 *= scale * sign;

                let dp = IVec3::new(dir.0, dir.1, len);
                branches.push((p + dp, len));
            }

            let colors = if false {
                vec!["c1", "c2", "c3", "c4", "c5", "c6"]
            } else {
                vec!["brown1", "brown2", "brown2", "brown3"]
            };

            for (q, len) in branches {
                let line = bresenham3d(p, q);
                for r in line {
                    let block = *rng.select(&colors);
                    trunk.set_voxel((r.x, r.y, r.z), block);
                }
                count += 1;
                base_points.push((q, len));
            }
        }
        segment += 1;
    }
    trunk
}

pub fn leaf_cluster(ctx: &GenContext, scene: &mut Scene2) -> VoxelSet {
    let mut voxel_set = VoxelSet::new();
    voxel_set.register_block(Block::color("green", 60, 25, 40));

    const R: i32 = 6;

    let mut rng = ctx.make_rng();

    let mut noise = rng.open_simplex().scale(0.25).build();

    for dz in -R..=R {
        for dy in -R..=R {
            for dx in -R..=R {
                let d = (dx.pow(2) + dy.pow(2) + dz.pow(2)) as f32;
                let r_scale = 0.25
                    + 0.75
                        * noise.gen_3d(
                            dx as f32 / R as f32,
                            dy as f32 / R as f32,
                            dz as f32 / R as f32,
                        );
                let r = r_scale * (R as f32);

                if d > r * r {
                    continue;
                }
                let p = ((dx.abs() % 2) + (dy.abs() % 2)) % 2;
                if p == (dz.abs() % 2) {
                    continue;
                }

                let name = "green";
                voxel_set.set_voxel((dx, dy, dz), name);
            }
        }
    }

    voxel_set
}
