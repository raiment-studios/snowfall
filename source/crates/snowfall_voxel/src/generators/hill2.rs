use crate::internal::*;

pub fn hill2(ctx: &GenContext, scene: &Scene2) -> VoxelSet {
    let mut rng = ctx.make_rng();

    let mut model = VoxelSet::new();
    model.register_block(Block::color("dirt", 25, 20, 10));
    model.register_block(Block::color("dirt2", 20, 15, 10));
    model.register_block(Block::color("grass1", 5, 60, 10));
    model.register_block(Block::color("grass2", 3, 45, 2));
    model.register_block(Block::color("red", 255, 0, 0));
    model.register_block(Block::color("blue", 0, 0, 255));

    use std::f32::consts::PI;
    const R: i32 = 127;
    let noise4 = rng.open_simplex().scale(1.0 / 2.0).build();
    let noise3 = rng.open_simplex().scale(1.0 / 1.5).build();
    let noise12 = rng.open_simplex().scale(1.0 / 32.0).build();
    let noise20 = rng.open_simplex().scale(1.0 / 1.0).build();

    let mut dirt_block = rng.select_fn(vec!["dirt", "dirt2"]);
    let mut grass_block = rng.select_fn(vec!["grass1", "grass2"]);

    for y in -R..=R {
        for x in -R..=R {
            let u0 = (x as f32) / (R as f32);
            let v0 = (y as f32) / (R as f32);

            let jitter_radius = 1.5 * noise4.gen_2d(u0, v0);
            let jitter_angle = 2.0 * PI * noise3.gen_2d(u0, v0);
            let (du, dv) = rotate_2d(jitter_radius, 0.0, jitter_angle);
            let u = u0 + du;
            let v = v0 + dv;

            let h0 = 16.0 * noise4.gen_2d(u, v).powf(1.0);
            let h1 = 32.0 * noise3.gen_2d(u, v).powf(4.0);
            let h = h0 + h1;

            // Smooth influnece to 0 around the tile edges
            let dist_norm = x.abs().max(y.abs()) as f32 / (R as f32);
            let scale = 1.0 - dist_norm.powf(16.0);
            let h = (h * scale).round() as i32;

            let dirt_patch = noise12.gen_2d(u0, v0) < noise20.gen_2d(u0, v0) * 0.9;
            let base_z = scene.terrain.height_at(x, y).unwrap_or(0);

            // Draw the voxels
            for z in 0..=h {
                let block = if z >= h && !dirt_patch {
                    grass_block()
                } else {
                    dirt_block()
                };
                model.set_voxel((x, y, base_z + z), block);
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
