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
