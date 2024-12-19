use crate::internal::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct Params {
    ground_type: Option<String>,
}

pub fn hill4(ctx: &GenContext, scene: &mut Scene2) -> VoxelSet {
    let mut rng = ctx.make_rng();

    let mut params: Params = ctx.params();
    let ground_type = params.ground_type.get_or_insert("grass".to_string());

    let mut model = VoxelSet::new();
    model.register_block(Block::color("dirt1", 10, 8, 4));
    model.register_block(Block::color("dirt2", 16, 12, 7));
    model.register_block(Block::color("grass1", 5, 60, 10));
    model.register_block(Block::color("grass2", 3, 45, 2));

    use std::f32::consts::PI;
    const R: i32 = 256;
    let noise4 = rng.open_simplex().scale(0.25).build();
    let noise3 = rng.open_simplex().scale(0.5).build();

    let mut gen_block = match ground_type.as_str() {
        "dirt" => rng.select_fn(vec!["dirt1", "dirt2"]),
        _ => rng.select_fn(vec!["grass1", "grass2"]),
    };

    for y in -R..=R {
        for x in -R..=R {
            let u0 = (x as f32) / (R as f32);
            let v0 = (y as f32) / (R as f32);

            let jitter_radius = 1.5 * noise4.gen_2d(u0, v0);
            let jitter_angle = 2.0 * PI * noise3.gen_2d(u0, v0);
            let h3 = 64.0 * jitter_radius * (0.5 + 0.5 * jitter_angle.cos());
            let h = h3.powf(1.15).max(1.0);

            for z in 1..=(h.round() as i32) {
                let block = gen_block();
                model.set_voxel((x, y, z), block);
            }
        }
    }

    model
}
