use crate::internal::*;

pub fn small_hill(seed: u64, ctx: &GenContext) -> VoxelSet {
    let mut rng = RNG::new(seed);

    let mut model = VoxelSet::new();
    model.register_block(Block::color("dirt", 25, 20, 10));
    model.register_block(Block::color("dirt2", 20, 15, 10));
    model.register_block(Block::color("grass1", 5, 60, 10));
    model.register_block(Block::color("grass2", 3, 45, 2));
    model.register_block(Block::color("red", 255, 0, 0));
    model.register_block(Block::color("blue", 0, 0, 255));

    let cos01 = |x: f32| (x.cos() + 1.0) / 2.0;

    const SIZE: f32 = 64.0;

    let size_x = (SIZE * rng.range(0.75..=1.25)).ceil() as i32;
    let size_y = (SIZE * rng.range(0.75..=1.25)).ceil() as i32;
    let power = rng.range(0.45..=0.55);
    let angle = rng.range(0.0..std::f32::consts::PI * 2.0);
    let mut dirt_block = rng.select_fn(vec!["dirt", "dirt2"]);
    let mut grass_block = rng.select_fn(vec!["grass1", "grass2"]);

    let noise = rng.open_simplex().scale(12.0).build();

    for y in -size_y..=size_y {
        for x in -size_x..=size_x {
            let u = (x as f32) / (size_x as f32);
            let v = (y as f32) / (size_y as f32);
            let u2 = u * angle.cos() - v * angle.sin();
            let v2 = u * angle.sin() + v * angle.cos();
            let u = u2 * 4.0;
            let v = v2 * 4.0;

            let h = 16.0 * cos01(u * 0.5).powf(power) * cos01(v * 0.5).powf(power) - 8.25;
            let h = 3.0 * h;
            let zh = h.floor() as i32;
            let base_z = ctx.ground_height_at(x, y).unwrap_or(0);

            for z in 0..=zh {
                let block = if z < zh {
                    "dirt"
                } else {
                    let v = noise.gen_3d(x as f32, y as f32, z as f32);
                    if v < 0.41 {
                        dirt_block()
                    } else {
                        grass_block()
                    }
                };
                model.set_voxel((x, y, base_z + z), block);
            }
        }
    }
    model
}
