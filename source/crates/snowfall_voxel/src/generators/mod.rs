mod bare_tree;
mod chest;
mod chest_and_key;
mod cloud;
mod cluster;
mod cluster2;
mod desolate_hill;
mod fence;
mod flat_ground;
mod flower;
mod hill2;
mod hill3;
mod hill4;
mod hill_with_road;
mod pine_tree;
mod road;
mod rocks;
mod small_hill;
mod tree1;
mod tree2;
mod tree_cluster;
mod tree_hill;

pub use bare_tree::*;
pub use chest::*;
pub use chest_and_key::*;
pub use cloud::*;
pub use cluster::*;
pub use cluster2::*;
pub use desolate_hill::*;
pub use fence::*;
pub use flat_ground::*;
pub use flower::*;
pub use hill2::*;
pub use hill3::*;
pub use hill4::*;
pub use hill_with_road::*;
pub use pine_tree::*;
pub use road::*;
pub use rocks::*;
pub use small_hill::*;
pub use tree1::*;
pub use tree2::*;
pub use tree_cluster::*;
pub use tree_hill::*;

use crate::internal::*;

pub fn generate_model(ctx: &GenContext, scene: &mut Scene2) -> VoxelModel {
    let seed = ctx.seed;
    match ctx.generator.as_str() {
        "bare_tree" => bare_tree(ctx, scene).into(),
        "chest" => chest(ctx, scene).into(),
        "chest_cluster" => chest_cluster(ctx, scene).into(),
        "cloud_cluster" => cloud_cluster(ctx, scene).into(),
        "chest_and_key" => chest_and_key(ctx, scene).into(),
        "flat_ground" => flat_ground(ctx, scene).into(),
        "cloud" => cloud(ctx, scene).into(),
        "cluster" => cluster(seed, ctx).into(),
        "cluster2" => cluster2(ctx, scene).into(),
        "desolate_hill" => desolate_hill(ctx, scene).into(),
        "fence" => fence(ctx, scene).into(),
        "flower_cluster" => flower_cluster(ctx, scene).into(),
        "flower_field" => flower_field(ctx, scene).into(),
        "flower" => flower(ctx, scene).into(),
        "hill_with_road" => hill_with_road(ctx, scene).into(),
        "hill2" => hill2(ctx, scene).into(),
        "hill3" => hill3(ctx, scene).into(),
        "hill4" => hill4(ctx, scene).into(),
        "pine_tree" => pine_tree(ctx, scene).into(),
        "road" => road(ctx, scene),
        "rocks" => rocks(ctx, scene),
        "small_hill" => small_hill(ctx, scene).into(),
        "tree_cluster" => tree_cluster(seed).into(),
        "tree_hill" => tree_hill(seed, scene).into(),
        "tree1" => tree1(ctx, scene).into(),
        "tree2" => tree2(ctx, scene).into(),
        _ => {
            panic!("unknown generator: {}", ctx.generator);
        }
    }
}
