use crate::internal::*;

pub fn desolate_hill(ctx: &GenContext, scene: &mut Scene2) -> Group {
    let mut rng = ctx.make_rng();

    let mut ctx = ctx.fork("hill4", rng.seed8());
    ctx.params = serde_json::json!({
        "ground_type": "dirt",
    });
    scene.terrain = generators::hill4(&ctx, scene);

    generators::rocks(&ctx.fork("rocks", rng.seed8()), scene);

    for _ in 0..2 {
        generators::road(&ctx.fork("road", rng.seed8()), scene);
    }

    let mut group = Group::new();

    const R: i32 = 255 - 32;
    for _ in 0..32 {
        let mut ctx = ctx.fork("cluster2", rng.seed8());
        ctx.center = IVec3::new(rng.range(-R..=R), rng.range(-R..=R), 0);
        ctx.params = serde_json::json!({
            "count": [3, 8],
            "range": 32,
            "generators": [
                (10, "bare_tree".to_string()), //
            ],
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

pub fn leaf_cluster(ctx: &GenContext, scene: &mut Scene2) -> VoxelSet {
    let mut voxel_set = VoxelSet::new();
    voxel_set.register_block(Block::color("green", 60, 25, 40));

    const R: i32 = 6;

    let mut rng = ctx.make_rng();
    let noise = rng.open_simplex().scale(0.25).build();

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
