use crate::internal::*;

pub fn tree1(_seed: u64) -> VoxelSet {
    let mut model = VoxelSet::new();
    model.register_block(Block::color("grass", 50, 200, 50));
    model.register_block(Block::color("sand", 180, 200, 20));
    model.register_block(Block::color("wood", 46, 38, 38));

    const R: i32 = 8;
    const H: i32 = 20;

    for z in 0..=H {
        model.set_voxel((0, 0, z), "wood");
    }

    for z in -R..=R {
        for y in -R..=R {
            for x in -R..=R {
                let d = (x.pow(2) + y.pow(2) + z.pow(2)) as f32;
                let r = R as f32;
                if d > r * r {
                    continue;
                }

                let p = ((x.abs() % 2) + (y.abs() % 2)) % 2;
                if p == (z.abs() % 2) {
                    continue;
                }
                let name = if (y.abs() % 2) == 0 { "grass" } else { "sand" };
                model.set_voxel((x, y, z + R / 2 + H), name);
            }
        }
    }
    model
}
