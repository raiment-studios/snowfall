use crate::internal::*;

pub struct VoxelScene {
    layers: Vec<Layer>,
}

pub struct Layer {
    objects: Vec<Object>,
}

/// TODO:
/// - Rotation (in 90 degree increments) and mirror
///   - Worth constraining rather than an arbitrary quaternion?
pub struct Object {
    storage: Storage,
    position: IVec3,
    model: Model,
}

pub enum Storage {
    Embedded,
    FileLink(String),
}

pub enum Model {
    FileRef(String),
    VoxelSet(VoxelSet),
    VoxelGrid(VoxelGrid),
}
