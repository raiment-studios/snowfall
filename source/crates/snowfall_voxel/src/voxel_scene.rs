use crate::internal::*;

/// 🔭 FUTURE
///
/// Consider whether it might be simpler / possible to use the
/// same struct for I/O and in-memory representation.
///
#[derive(Serialize, Deserialize)]
pub struct VoxelScene {
    pub layers: Vec<Layer>,
}

impl VoxelScene {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn add_object(&mut self, layer: usize, object: Object) {
        self.layers.resize_with(layer + 1, || Layer {
            objects: Vec::new(),
        });
        self.layers[layer].objects.push(object);
    }
}

#[derive(Serialize, Deserialize)]
pub struct Layer {
    pub objects: Vec<Object>,
}

/// TODO:
///
/// - Rotation (in 90 degree increments) and mirror
///   - Worth constraining rather than an arbitrary quaternion?
#[derive(Serialize, Deserialize)]
pub struct Object {
    pub model_id: String,
    pub seed: u64,
    pub position: IVec3,
}

pub const VOXEL_SCENE_FILE_IDENTIFIER: &str = "SNOWFALL_VOXEL_SCENE";
pub const VOXEL_SCENE_FILE_VERSION: &str = "0.0.1";

#[derive(Serialize, Deserialize)]
pub struct VoxelSceneFile {
    pub identifier: String,
    pub version: String,
    pub scene: VoxelScene,
}

impl VoxelSceneFile {
    pub fn new(scene: VoxelScene) -> Self {
        Self {
            identifier: VOXEL_SCENE_FILE_IDENTIFIER.to_string(),
            version: VOXEL_SCENE_FILE_VERSION.to_string(),
            scene,
        }
    }
}
