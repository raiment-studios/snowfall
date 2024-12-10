use snowfall_voxel::prelude::*;

fn main() {
    let model = generate_model();

    for voxel in model.voxel_iter(false) {
        println!("{:?}", voxel);
    }

    model.serialize_to_file("content/model.bin");
}

fn generate_model() -> VoxelSet {
    let mut model = VoxelSet::new();
    model.register_block(Block::color("grass", 50, 200, 50));
    model.register_block(Block::color("sand", 180, 200, 20));

    const R: i32 = 8;
    for z in -R..=R {
        for y in -R..=R {
            for x in -R..=R {
                let d = (x.pow(2) + y.pow(2) + z.pow(2)) as f32;
                if d > R as f32 * R as f32 {
                    continue;
                }

                let p = ((x.abs() % 2) + (y.abs() % 2)) % 2;
                if p == (z.abs() % 2) {
                    continue;
                }
                let name = if (y.abs() % 2) == 0 { "grass" } else { "sand" };
                model.set_voxel((x, y, z + R / 2), name);
            }
        }
    }
    model
}
