use crate::internal::*;

pub enum VoxelModel {
    Empty,
    VoxelSet(Box<VoxelSet>),
    VoxelScene(Box<VoxelScene>),
}

impl Into<VoxelModel> for VoxelSet {
    fn into(self) -> VoxelModel {
        VoxelModel::VoxelSet(Box::new(self))
    }
}

impl Into<VoxelModel> for VoxelScene {
    fn into(self) -> VoxelModel {
        VoxelModel::VoxelScene(Box::new(self))
    }
}

/// Provides the parameters needed to generate a particular
/// model dynamically (or from disk).
///
// TODO:
// - rotation in 90 degree increments
// - mirror
pub struct VoxelModelRef {
    pub model_id: String,
    pub seed: u64,
    pub params: serde_json::Value,
    pub position: IVec3,
}
