mod cluster;
mod fence;
mod hill2;
mod hill3;
mod hill_with_road;
mod maple;
mod pine_tree;
mod small_hill;
mod tree1;
mod tree2;
mod tree_cluster;
mod tree_hill;

pub use cluster::*;
pub use fence::*;
pub use hill2::*;
pub use hill3::*;
pub use hill_with_road::*;
pub use maple::*;
pub use pine_tree::*;
pub use small_hill::*;
pub use tree1::*;
pub use tree2::*;
pub use tree_cluster::*;
pub use tree_hill::*;

use crate::internal::*;

pub fn generate_model(ctx: &GenContext, scene: &mut Scene2) -> VoxelModel {
    let seed = ctx.seed;
    match ctx.generator.as_str() {
        "cluster" => cluster(seed, ctx).into(),
        "fence" => fence(ctx, scene).into(),
        "flower" => maple(ctx, scene).into(),
        "hill_with_road" => hill_with_road(ctx, scene).into(),
        "hill2" => hill2(ctx, scene).into(),
        "hill3" => hill3(ctx, scene).into(),
        "pine_tree" => pine_tree(ctx, scene).into(),
        "small_hill" => small_hill(ctx, scene).into(),
        "tree_cluster" => tree_cluster(seed).into(),
        "tree_hill" => tree_hill(seed, scene).into(),
        "tree1" => tree1(ctx, scene).into(),
        "tree2" => tree2(ctx, scene).into(),
        _ => VoxelModel::Empty,
    }
}
