use snowfall_core::prelude::*;
use snowfall_voxel::prelude::*;

fn main() {
    let seed = RNG::generate_seed() % 8192;
    let table = vec![
        ("tree1", generate_tree1(seed)), //
        ("tree2", generate_tree2(seed)), //
    ];

    println!("Generating models (seed={})...", seed);
    for (name, model) in table {
        model.serialize_to_file(&format!("content/{}.bin", name));
        println!("  generated: {}", name);
    }
    println!("done.")
}

fn generate_tree2(seed: u64) -> VoxelSet {
    let mut rng = RNG::new(seed);

    let mut model = VoxelSet::new();
    model.register_block(Block::color("leaves", 40, 180, 30));
    model.register_block(Block::color("leaves2", 30, 150, 30));
    model.register_block(Block::color("leaves3", 230, 150, 30));
    model.register_block(Block::color("sand", 180, 200, 20));
    model.register_block(Block::color("wood", 46, 38, 38));
    model.register_block(Block::color("birch_wood0", 200, 205, 203));
    model.register_block(Block::color("birch_wood1", 180, 185, 173));
    model.register_block(Block::color("birch_wood2", 30, 35, 33));

    let tree_type = *rng.select(&vec!["standard", "birch"]);

    let mut wood_select = {
        let mut rng = rng.fork();
        move || match tree_type {
            "birch" => *rng.select_weighted(&vec![
                (40, "birch_wood0"),
                (40, "birch_wood1"),
                (20, "birch_wood2"),
            ]),
            _ => "wood",
        }
    };
    let mut leaf_select = {
        let mut rng = rng.fork();
        move || match tree_type {
            "birch" => *rng.select(&vec!["leaves", "leaves2"]),
            _ => *rng.select(&vec!["leaves", "leaves2", "leaves3"]),
        }
    };

    const R: i32 = 8;
    let height: i32 = rng.range(12..=20);

    for z in 0..=height {
        let block_name = wood_select();
        model.set_voxel((0, 0, z), block_name);
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
                let name = leaf_select();
                model.set_voxel((x, y, z + R / 2 + height), name);
            }
        }
    }
    model
}

fn generate_tree1(_seed: u64) -> VoxelSet {
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
