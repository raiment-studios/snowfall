use crate::internal::*;

pub fn pine_tree(seed: u64, ctx: &GenContext) -> VoxelSet {
    let mut rng = RNG::new(seed);

    let mut model = VoxelSet::new();
    model.register_block(Block::color("leaves", 10, 140, 30));
    model.register_block(Block::color("leaves2", 30, 90, 30));
    model.register_block(Block::color("leaves3", 30, 70, 30));
    model.register_block(Block::color("wood", 46, 38, 38));
    model.register_block(Block::color("wood2", 26, 28, 28));
    model.register_block(Block::color("wood3", 36, 38, 31));

    const R: i32 = 8;
    let base_height: i32 = rng.range(8..=12);
    let cone_height: i32 = base_height + rng.range(4..=16);
    let girth: f32 = rng.range(0.5..=0.75);

    let mut leaf_select = rng.select_fn(vec!["leaves", "leaves2", "leaves3"]);
    let mut wood_select = rng.select_fn(vec!["wood", "wood2", "wood3"]);

    let base_z = ctx.ground_height_at(0, 0).unwrap_or(0);

    for z in base_z..=base_z + base_height {
        model.set_voxel((0, 0, z), wood_select());
    }

    for z in 0..=cone_height {
        let r = (cone_height - z) + 1;
        let r = (r as f32).powf(girth).ceil() as i32;
        for y in -r..=r {
            for x in -r..=r {
                let d = (x.pow(2) + y.pow(2)) as f32;
                if d > r as f32 * r as f32 {
                    continue;
                }
                let p = ((x.abs() % 2) + (y.abs() % 2)) == 1;
                if p == ((z.abs() % 2) == 1) {
                    continue;
                }

                let block = leaf_select();
                model.set_voxel((x, y, z + base_height + base_z), block);
            }
        }
    }
    model
}
