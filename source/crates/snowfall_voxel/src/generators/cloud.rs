use crate::internal::*;

pub fn cloud_cluster(ctx: &GenContext, scene: &mut Scene2) -> Group {
    let mut rng = ctx.make_rng();

    const R: i32 = 200;

    let mut group = Group::new();
    for _ in 0..14 {
        let mut ctx = ctx.fork("cloud", rng.seed8());
        ctx.center = IVec3::new(rng.range(-R..=R), rng.range(-R..=R), rng.range(96..=162));

        let set = cloud(&ctx, scene);
        group.objects.push(Object {
            generator_id: ctx.generator.clone(),
            seed: rng.seed8(),
            params: ctx.params.clone(),
            position: ctx.center,
            imp: ObjectImp::VoxelSet(Box::new(set)),
        });
    }
    group
}

pub fn cloud(ctx: &GenContext, scene: &mut Scene2) -> VoxelSet {
    use std::f32::consts::PI;

    let mut voxel_set = VoxelSet::new();
    voxel_set.register_block(Block::color("cloud1", 132, 137, 144));
    voxel_set.register_block(Block::color("cloud2", 227, 227, 227));
    voxel_set.register_block(Block::color("red", 255, 0, 0));

    const R: i32 = 14;
    const R2: i32 = 8;

    let mut rng = ctx.make_rng();

    let count = rng.range(6..=16);

    for _ in 0..count {
        let noise = rng.open_simplex().scale(0.25).build();
        let noise2 = rng.open_simplex().scale(0.15).build();

        let angle = rng.range(-PI / 5.0..=PI / 5.0);
        let range_x = ((R as f32) * rng.range(1.0..2.0)).round() as i32;
        let range_y = ((R as f32) * rng.range(1.0..4.0)).round() as i32;
        let range_z = ((R as f32) * rng.range(0.25..1.25)).round() as i32;

        let offset_x = rng.range(-R2..=R2);
        let offset_y = rng.range(-R2..=R2);
        let offset_z = rng.range(-R2 / 2..=R2 / 2);

        for dz in -range_x..=range_x {
            for dy in -range_y..=range_y {
                for dx in -range_z..=range_z {
                    let u = dx as f32 / range_x as f32;
                    let v = dy as f32 / range_y as f32;
                    let w = dz as f32 / range_z as f32;

                    let d2 = u * u + v * v + w * w;
                    let r = 0.25 + 0.75 * noise.gen_3d(u, v, w);
                    if d2 > r * r {
                        continue;
                    }

                    let n = noise2.gen_3d(u, v, w);
                    let block = if n < 0.35 {
                        "cloud1"
                    } else if n < 0.62 {
                        "cloud2"
                    } else {
                        "empty"
                    };

                    let (rx, ry) = rotate_2d(dx as f32, dy as f32, angle);
                    let dx = rx.round() as i32;
                    let dy = ry.round() as i32;

                    voxel_set.set_voxel((dx + offset_x, dy + offset_y, dz + offset_z), block);
                }
            }
        }
    }

    voxel_set
}
