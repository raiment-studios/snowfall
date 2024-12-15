use bevy_math::IVec3;
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

    let ctx = GenContext::new();
    let model: VoxelModel = generate_model(generator.as_str(), seed, &ctx);
    match model {
        VoxelModel::Empty => {
            eprintln!("Unknown generator: {}", generator);
            std::process::exit(1);
        }
        VoxelModel::VoxelSet(model) => {
            model.serialize_to_file(&format!("content/{}-{}.bin", generator, seed));
        }
        VoxelModel::VoxelScene(model) => {
            let filename = format!("content/{}-{}.yaml", generator, seed);
            let file = VoxelSceneFile::new(*model);
            serde_yaml::to_writer(std::fs::File::create(&filename).unwrap(), &file).unwrap();
        }
    }
    println!("Generated model {} (seed={})", generator, seed);
}
