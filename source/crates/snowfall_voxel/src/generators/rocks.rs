use crate::internal::*;

pub fn rocks(ctx: &GenContext, scene: &mut Scene2) -> VoxelModel {
    let model = &mut scene.terrain;
    let stone1 =
        model.register_block(Block::color("stone1", 10, 10, 11).modify(|b| b.occupied = true));
    let stone2 =
        model.register_block(Block::color("stone2", 5, 6, 5).modify(|b| b.occupied = true));

    let mut rng = ctx.make_rng();

    let noise0 = rng.open_simplex().scale(0.125).build();
    let noise1 = rng.open_simplex().scale(0.055).build();
    let noise2 = rng.open_simplex().scale(0.025).build();

    let noise_stone = |u, v| {
        let n = noise2.gen_2d(u, v).max(noise1.gen_2d(u, v))
            * if noise0.gen_2d(u, v) > 0.45 { 0.0 } else { 1.0 };
        n
    };
    let mut gen_block = rng.select_fn(vec![stone1, stone2]);

    const R: i32 = 256;
    for y in -R..=R {
        for x in -R..=R {
            let u = (x as f32) / (R as f32);
            let v = (y as f32) / (R as f32);

            let n = noise_stone(u, v);
            if n < 0.65 {
                continue;
            }
            let h = 1.0 + 60.0 * (n - 0.65);

            let base_z = model.height_at(x, y).unwrap_or(1);
            for z in 1..=(h.round() as i32) {
                let block = gen_block();
                model.set((x, y, base_z + z), block);
            }
        }
    }
    VoxelModel::Empty
}
