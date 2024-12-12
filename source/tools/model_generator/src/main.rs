use bevy_math::prelude::*;
use snowfall_core::prelude::*;
use snowfall_voxel::prelude::*;

use clap::Parser;

#[derive(Parser, Debug)]
struct ProcessArgs {
    /// Name of the generator to use
    generator: String,
    /// Seed to use for generation
    seed: u64,
}

fn main() {
    let ProcessArgs { generator, seed } = ProcessArgs::parse();

    let model: ModelType = match generator.as_str() {
        "tree1" => generate_tree1(seed).into(),
        "tree2" => generate_tree2(seed).into(),
        "pine_tree" => generate_pine_tree(seed).into(),
        "small_hill" => generate_small_hill(seed).into(),

        "tree_cluster" => generate_tree_cluster(seed).into(),

        _ => ModelType::Empty,
    };

    match model {
        ModelType::Empty => {
            eprintln!("Unknown generator: {}", generator);
            std::process::exit(1);
        }
        ModelType::VoxelSet(model) => {
            model.serialize_to_file(&format!("content/{}-{}.bin", generator, seed));
        }
        ModelType::VoxelScene(model) => {
            let filename = format!("content/{}-{}.yaml", generator, seed);
            let file = VoxelSceneFile::new(model);
            serde_yaml::to_writer(std::fs::File::create(&filename).unwrap(), &file).unwrap();
        }
    }
    println!("Generated model {} (seed={})", generator, seed);
}

enum ModelType {
    Empty,
    VoxelSet(VoxelSet),
    VoxelScene(VoxelScene),
}

impl Into<ModelType> for VoxelSet {
    fn into(self) -> ModelType {
        ModelType::VoxelSet(self)
    }
}

impl Into<ModelType> for VoxelScene {
    fn into(self) -> ModelType {
        ModelType::VoxelScene(self)
    }
}

fn generate_tree_cluster(seed: u64) -> VoxelScene {
    let mut rng = RNG::new(seed);

    const MAX_ATTEMPTS: usize = 128;
    const CLOSEST_DISTANCE: f32 = 12.0;
    const RANGE: i32 = 48;
    let mut count = rng.range(12..=24);

    let mut scene = VoxelScene::new();

    let mut point_set = PointSet::new();
    for _ in 0..MAX_ATTEMPTS {
        let model_id = *rng.select_weighted(&vec![
            (10, "tree1"), //
            (10, "tree2"),
            (80, "pine_tree"),
        ]);
        let seed = rng.range(1..8192);
        let position = IVec3::new(rng.range(-RANGE..=RANGE), rng.range(-RANGE..=RANGE), 0);

        let d = point_set.nearest_distance(&position).unwrap_or(f32::MAX);
        if d < CLOSEST_DISTANCE {
            continue;
        }

        point_set.add(position);
        scene.add_object(
            0,
            Object {
                model_id: model_id.to_string(),
                seed,
                position,
            },
        );
        count -= 1;
        if count == 0 {
            break;
        }
    }
    scene
}

fn generate_small_hill(seed: u64) -> VoxelSet {
    let mut rng = RNG::new(seed);

    let mut model = VoxelSet::new();
    model.register_block(Block::color("dirt", 50, 40, 20));
    model.register_block(Block::color("dirt2", 60, 40, 20));

    let cos01 = |x: f32| (x.cos() + 1.0) / 2.0;

    const SIZE: f32 = 16.0;

    let size_x = (SIZE * rng.range(0.75..=1.25)).ceil() as i32;
    let size_y = (SIZE * rng.range(0.75..=1.25)).ceil() as i32;
    let power = rng.range(0.45..=0.55);
    let angle = rng.range(0.0..std::f32::consts::PI * 2.0);
    let mut dirt_block = rng.select_fn(vec!["dirt", "dirt2"]);

    for y in -size_y..=size_y {
        for x in -size_x..=size_x {
            let u = (x as f32) / (size_x as f32);
            let v = (y as f32) / (size_y as f32);
            let u2 = u * angle.cos() - v * angle.sin();
            let v2 = u * angle.sin() + v * angle.cos();
            let u = u2 * 4.0;
            let v = v2 * 4.0;

            let h = 16.0 * cos01(u * 0.5).powf(power) * cos01(v * 0.5).powf(power) - 8.25;
            let zh = h.floor() as i32;

            for z in 0..=zh {
                model.set_voxel((x, y, z), dirt_block());
            }
        }
    }

    model
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

    let mut leaf_select = rng.select_fn(vec!["leaves", "leaves2", "leaves3"]);
    let mut wood_select = rng.select_fn(vec!["wood", "wood2", "wood3"]);

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
                let p = ((x.abs() % 2) + (y.abs() % 2)) == 1;
                if p == ((z.abs() % 2) == 1) {
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
