use crate::internal::*;

/// VoxelSet is simplified voxel representation designed for smaller models
/// that are bounded and can have all chunks loaded into memory at once.
///
/// This is a complementary structure to VoxelGrid, designed more for individual
/// models or small scenes rather than unbounded terrain data.
pub struct VoxelSet {
    generation: u64,     // Generation number used to track changes
    palette: Vec<Block>, // Palette of blocks used in the set
    data: HashMap<VSVec3, u16>,
}

/// VSVec3 = Voxel-space Vector3
struct VSVec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl VSVec3 {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        VSVec3 { x, y, z }
    }

    pub fn from_world(wx: f32, wy: f32, wz: f32) -> Self {
        VSVec3 {
            x: wx.floor() as i32,
            y: wy.floor() as i32,
            z: wz.floor() as i32,
        }
    }
}

impl From<(i32, i32, i32)> for VSVec3 {
    fn from(v: (i32, i32, i32)) -> Self {
        VSVec3::new(v.0, v.1, v.2)
    }
}
