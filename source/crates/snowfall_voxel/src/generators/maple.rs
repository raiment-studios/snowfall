use crate::internal::*;

pub fn maple(ctx: &GenContext, scene: &mut Scene2) -> Group {
    let mut rng = ctx.make_rng();

    let mut group = Group::new();
    let mut clusters = Vec::new();
    const RANGE: i32 = 7;
    for _ in 0..6 {
        let position = IVec3::new(
            rng.range(-RANGE..=RANGE),
            rng.range(-RANGE..=RANGE),
            rng.range(15..=20),
        );
        clusters.push(position.clone());

        let seed = rng.seed8();
        ctx.fork("leaf_cluster", seed);

        let voxel_set = leaf_cluster(ctx, scene);
        group.objects.push(Object {
            generator_id: "leaf_cluster".to_string(),
            seed,
            position,
            params: serde_json::Value::Null,
            imp: ObjectImp::VoxelSet(Box::new(voxel_set)),
        });
    }

    let mut trunk = VoxelSet::new();
    trunk.register_block(Block::color("brown", 60, 40, 20));

    for p in clusters {
        let base_z = ((p.z as f32) * rng.range(0.15..=0.5)).round() as i32;

        let line = bresenham3d((0, 0, 0).into(), (0, 0, base_z).into());
        for q in line {
            trunk.set_voxel((q.x, q.y, q.z), "brown");
        }

        let line = bresenham3d((0, 0, base_z).into(), p);
        for q in line {
            trunk.set_voxel((q.x, q.y, q.z), "brown");
        }
    }

    group.objects.push(Object {
        generator_id: "trunk".to_string(),
        seed: 0,
        position: IVec3::ZERO.clone(),
        params: serde_json::Value::Null,
        imp: ObjectImp::VoxelSet(Box::new(trunk)),
    });

    group
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
