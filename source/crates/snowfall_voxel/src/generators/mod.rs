mod fence;
mod hill2;
mod pine_tree;
mod small_hill;
mod tree1;
mod tree2;
mod tree_cluster;
mod tree_hill;

pub use fence::*;
pub use hill2::*;
pub use pine_tree::*;
pub use small_hill::*;
pub use tree1::*;
pub use tree2::*;
pub use tree_cluster::*;
pub use tree_hill::*;

use crate::internal::*;

pub fn generate_model(model_id: &str, seed: u64, ctx: &GenContext) -> ModelType {
    match model_id {
        "tree1" => tree1(seed).into(),
        "tree2" => tree2(seed).into(),
        "pine_tree" => pine_tree(seed, ctx).into(),
        "small_hill" => small_hill(seed, ctx).into(),
        "fence" => fence(seed, ctx).into(),
        "hill2" => hill2(seed, ctx).into(),

        "tree_cluster" => tree_cluster(seed).into(),
        "tree_hill" => tree_hill(seed).into(),

        _ => ModelType::Empty,
    }
}
