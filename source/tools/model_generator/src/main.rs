use snowfall_core::prelude::*;
use snowfall_voxel::prelude::*;

fn main() {
    let seed = RNG::generate_seed() % 8192;
    let table = vec![
        ("tree1", generate_tree1(seed)),         //
        ("tree2", generate_tree2(seed)),         //
        ("pine_tree", generate_pine_tree(seed)), //
    ];

    println!("Generating models (seed={})...", seed);
    for (name, model) in table {
        model.serialize_to_file(&format!("content/{}.bin", name));
        println!("  generated: {}", name);
    }
    println!("done.")
}

fn generate_pine_tree(seed: u64) -> VoxelSet {
    let mut rng = RNG::new(seed);

    let mut model = VoxelSet::new();
    model.register_block(Block::color("leaves", 10, 140, 30));
    model.register_block(Block::color("leaves2", 30, 90, 30));
    model.register_block(Block::color("leaves3", 30, 70, 30));
    model.register_block(Block::color("wood", 46, 38, 38));
    model.register_block(Block::color("wood2", 26, 28, 28));
    model.register_block(Block::color("wood3", 36, 38, 31));

    const R: i32 = 8;
    let base_height: i32 = rng.range(4..=8);
    let cone_height: i32 = (base_height + rng.range(1..=6)).max(8);
    let girth: f32 = rng.range(0.5..=0.75);

    let mut leaf_select = {
        let mut rng = rng.fork();
        move || *rng.select(&vec!["leaves", "leaves2", "leaves3"])
    };

    let mut wood_select = {
        let mut rng = rng.fork();
        move || *rng.select(&vec!["wood", "wood2", "wood3"])
    };

    for z in 0..=base_height {
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

                let p = ((x.abs() % 2) + (y.abs() % 2)) % 2;
                if p == (z.abs() % 2) {
                    continue;
                }

                let block = leaf_select();
                model.set_voxel((x, y, z + base_height), block);
            }
        }
    }
    model
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
    let noise = rng.open_simplex().scale(3.0).build();

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

                let v = noise.gen_3d(x as f32, y as f32, z as f32);
                if v < 0.40 {
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
