/// VoxelGrid is a 3D grid of voxels designed for handling unbounded, sparse
/// voxel data.
///
/// As such it is designed with the following in mind:
///
/// - Chunks of the grid can be paged in / out of memory at any time
/// - The grid data structures should be minimal in size
/// - There may be a large number of empty chunks for things like the sky
///
/// It is designed primary for terrain or very large models
pub struct VoxelGrid {}
