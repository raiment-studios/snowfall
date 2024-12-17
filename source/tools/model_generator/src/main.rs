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

    let mut scene = Scene2::new();
    let ctx = GenContext::new(generator, seed);
    let model: VoxelModel = generate_model(&ctx, &mut scene);
    match model {
        VoxelModel::Empty => {
            eprintln!("Unknown generator: {}", ctx.generator);
            std::process::exit(1);
        }
        VoxelModel::VoxelSet(model) => {
            model.serialize_to_file(&format!("content/{}-{}.bin", ctx.generator, seed));
        }
        VoxelModel::VoxelScene(model) => {
            let filename = format!("content/{}-{}.yaml", ctx.generator, seed);
            let file = VoxelSceneFile::new(*model);
            serde_yaml::to_writer(std::fs::File::create(&filename).unwrap(), &file).unwrap();
        }
        _ => {
            eprintln!("Unsupported model type: {}", ctx.generator);
            std::process::exit(1);
        }
    }
    println!("Generated model {} (seed={})", ctx.generator, seed);
}
