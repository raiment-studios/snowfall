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

    // ------------------------------------------------------------------------
    // Block palette
    // ------------------------------------------------------------------------

    pub fn register_block(&mut self, block: Block) {
        let id = self.palette.len() as u16;
        self.palette.push(block);
    }

    // ------------------------------------------------------------------------
    // Voxel properties
    // ------------------------------------------------------------------------

    /// Returns the inclusive bounds of the voxel set.
    pub fn bounds(&self) -> (VSVec3, VSVec3) {
        let mut min = VSVec3::new(i32::MAX, i32::MAX, i32::MAX);
        let mut max = VSVec3::new(i32::MIN, i32::MIN, i32::MIN);
        for (vc, _) in self.voxel_iter(false) {
            min.x = min.x.min(vc.x);
            min.y = min.y.min(vc.y);
            min.z = min.z.min(vc.z);
            max.x = max.x.max(vc.x);
            max.y = max.y.max(vc.y);
            max.z = max.z.max(vc.z);
        }
        (min, max)
    }

    // ------------------------------------------------------------------------
    // Voxel manipulation
    // ------------------------------------------------------------------------

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

    // ------------------------------------------------------------------------
    // Serialization
    // ------------------------------------------------------------------------

    pub fn serialize_to_file(&self, path: &str) {
        let file = VoxelSetFile {
            identifier: VOXEL_SET_FILE_IDENTIFIER,
            version: [0, 0, 1, 0],
            compressed_voxel_set: serialize_and_compress(self),
        };
        let Ok(bytes) = serialize_to_bytes(&file) else {
            panic!("Failed to serialize voxel set");
        };
        std::fs::write(path, &bytes).expect("Failed to write file");
    }

    pub fn deserialize_from_file(path: &str) -> Result<Self, Error> {
        // Read the file at path as a byte array
        let bytes = std::fs::read(path).unwrap();
        let voxel_file = deserialize_from_bytes::<VoxelSetFile>(&bytes)?;

        // At this stage in development, we're not worried about backwards compatibility
        // so panic!() on anything we don't recognize.
        if voxel_file.identifier != VOXEL_SET_FILE_IDENTIFIER {
            return Err(Error::FileHeader(
                String::from_utf8_lossy(&voxel_file.identifier).to_string(),
            ));
        }
        if voxel_file.version != [0, 0, 1, 0] {
            return Err(Error::FileVersion(format!("{:?}", voxel_file.version)));
        }
        let voxel_set: Self = decompress_and_deserialize(&voxel_file.compressed_voxel_set);
        Ok(voxel_set)
    }
}

const VOXEL_SET_FILE_IDENTIFIER: [u8; 8] = *b"SNVSET\0\0";

#[derive(Serialize, Deserialize)]
struct VoxelSetFile {
    identifier: [u8; 8],
    version: [u8; 4],
    compressed_voxel_set: Vec<u8>,
}
