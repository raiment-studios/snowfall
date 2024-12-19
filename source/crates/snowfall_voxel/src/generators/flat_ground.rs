use crate::internal::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct Params {
    ground_type: Option<String>,
}

pub fn flat_ground(ctx: &GenContext, scene: &mut Scene2) -> VoxelSet {
    let mut rng = ctx.make_rng();

    let mut params: Params = ctx.params();
    let ground_type = params.ground_type.get_or_insert("grass".to_string());

    let mut model = VoxelSet::new();
    model.register_block(Block::color("dirt1", 10, 8, 4));
    model.register_block(Block::color("dirt2", 16, 12, 7));
    model.register_block(Block::color("grass1", 5, 60, 10));
    model.register_block(Block::color("grass2", 3, 45, 2));

    const R: i32 = 256;

    let mut gen_block = match ground_type.as_str() {
        "dirt" => rng.select_fn(vec!["dirt1", "dirt2"]),
        _ => rng.select_fn(vec!["grass1", "grass2"]),
    };

    for y in -R..=R {
        for x in -R..=R {
            let block = gen_block();
            model.set_voxel((x, y, 1), block);
        }
    }

    model
}
