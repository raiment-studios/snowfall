use crate::internal::*;
use snowfall_core::prelude::*;

/// VoxelSet is simplified voxel representation designed for smaller models
/// that are bounded and can have all chunks loaded into memory at once.
///
/// This is a complementary structure to VoxelGrid, designed more for individual
/// models or small scenes rather than unbounded terrain data.  It priorities
/// ease-of-use for small models over performance and scalability.
///
#[derive(Serialize, Deserialize)]
pub struct VoxelSet {
    generation: u64,     // Generation number used to track changes
    palette: Vec<Block>, // Palette of blocks used in the set
    data: HashMap<VSVec3, u16>,
}

impl VoxelSet {
    pub fn new() -> Self {
        let mut palette = Vec::new();
        palette.push(Block::empty());

        VoxelSet {
            generation: 0,
            palette,
            data: HashMap::new(),
        }
    }

    pub fn serialize_to_file(&self, path: &str) {
        let bytes = serialize_and_compress(&self);
        std::fs::write(path, &bytes).expect("Failed to write file");
    }

    pub fn deserialize_from_file(path: &str) -> Self {
        // Read the file at path as a byte array
        let bytes = std::fs::read(path).unwrap();
        let voxel_set = decompress_and_deserialize(&bytes);
        voxel_set
    }

    pub fn register_block(&mut self, block: Block) {
        let id = self.palette.len() as u16;
        self.palette.push(block);
    }

    pub fn set_voxel<S, T>(&mut self, vc: S, id: T)
    where
        S: Into<VSVec3>,
        T: Into<String>,
    {
        let id = id.into();
        let index = match self.palette.iter().position(|b| b.id == id) {
            Some(i) => i as u16,
            None => 0,
        };
        self.data.insert(vc.into(), index);
    }

    pub fn voxel_iter(&self, include_empty: bool) -> impl Iterator<Item = (VSVec3, &Block)> {
        self.data
            .iter()
            .filter(move |(_, &index)| include_empty || index != 0)
            .map(move |(vc, &index)| {
                let block = &self.palette[index as usize];
                (*vc, block)
            })
    }
}
