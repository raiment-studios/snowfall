use crate::internal::*;

pub enum VoxelModel {
    Empty,
    VoxelSet(Box<VoxelSet>),
    VoxelScene(Box<VoxelScene>),
    Group(Box<Group>),
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

impl Into<VoxelModel> for Group {
    fn into(self) -> VoxelModel {
        VoxelModel::Group(Box::new(self))
    }
}
