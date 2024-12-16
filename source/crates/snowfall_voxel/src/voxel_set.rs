use crate::internal::*;
use bevy_math::{Vec2, Vec3};
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

    // Storing the data by z-column is *much* faster in any context where
    // "height at x,y" is a common operation.
    data: HashMap<(i32, i32), HashMap<i32, u16>>,
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

    /// Adds the block to the palette for this voxel set.  If a block with the
    /// same name already exists, it will **replace** that definition.
    pub fn register_block(&mut self, block: Block) {
        if let Some(index) = self.palette.iter().position(|b| b.id == block.id) {
            self.palette[index] = block;
        } else {
            self.palette.push(block);
        }
    }

    // ------------------------------------------------------------------------
    // Voxel properties
    // ------------------------------------------------------------------------

    /// Returns the inclusive bounds of the voxel set.
    pub fn bounds(&self) -> (IVec3, IVec3) {
        let mut min = IVec3::new(i32::MAX, i32::MAX, i32::MAX);
        let mut max = IVec3::new(i32::MIN, i32::MIN, i32::MIN);
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

    // Returns the z-coordinate of the highest non-empty voxel
    // at x,y.  Returns None if there are no non-empty voxels at
    // that x,y coordinate.
    pub fn height_at(&self, x: i32, y: i32) -> Option<i32> {
        let Some(column) = self.data.get(&(x, y)) else {
            return None;
        };

        let mut height: Option<i32> = None;
        for (z, id) in column.iter() {
            if *id == 0 {
                continue;
            }
            match height {
                Some(h) => {
                    height = Some(h.max(*z));
                }
                None => {
                    height = Some(*z);
                }
            };
        }
        height
    }

    // ------------------------------------------------------------------------
    // Voxel manipulation
    // ------------------------------------------------------------------------

    pub fn is_empty(&self, vc: IVec3) -> bool {
        let Some(col) = self.data.get(&(vc.x, vc.y)) else {
            return true;
        };
        *col.get(&vc.z).unwrap_or(&0) == 0
    }

    pub fn is_empty_f32(&self, x: f32, y: f32, z: f32) -> bool {
        self.is_empty(from_ws(x, y, z))
    }

    pub fn set_voxel<S, T>(&mut self, vc: S, id: T)
    where
        S: Into<IVec3>,
        T: Into<String>,
    {
        let id = id.into();
        let index = match self.palette.iter().position(|b| b.id == id) {
            Some(i) => i as u16,
            _ => 0,
        };
        // self.data.insert(vc.into(), index);

        // Get the column or create it
        let vc = vc.into();
        let column = self.data.entry((vc.x, vc.y)).or_insert(HashMap::new());
        column.insert(vc.z, index);
    }

    pub fn voxel_iter(&self, include_empty: bool) -> Vec<(IVec3, &Block)> {
        // Collect all the voxels into a vec
        let mut voxels = Vec::new();
        for (x, column) in self.data.iter() {
            for (z, id) in column.iter() {
                let vc = IVec3::new(x.0, x.1, *z);
                let block = &self.palette[*id as usize];
                if block.is_empty() && !include_empty {
                    continue;
                }
                voxels.push((vc, block));
            }
        }
        voxels
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

pub struct VoxelMesh {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub colors: Vec<[f32; 4]>,
}

pub fn build_mesh_arrays(voxel_set: &VoxelSet) -> VoxelMesh {
    let bounds = voxel_set.bounds();
    let max_voxel_count = (bounds.1.x - bounds.0.x + 1)
        * (bounds.1.y - bounds.0.y + 1)
        * (bounds.1.z - bounds.0.z + 1);
    let count = max_voxel_count as usize;

    // Over-allocate (and shrink when we're done)
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(8 * count);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(8 * count);
    let mut colors: Vec<[f32; 4]> = Vec::with_capacity(8 * count);

    // Downward facing triangles on Z = 0
    let tri_points = [
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
    ];

    let face_normals = [
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    ];

    for (position, voxel) in voxel_set.voxel_iter(false) {
        let offset: Vec3 = Vec3::new(position.x as f32, position.y as f32, position.z as f32);
        if voxel.is_empty() {
            continue;
        }

        let rgba = match voxel.shader {
            BlockShader::RGB(ref rgb) => [
                rgb.r as f32 / 255.0,
                rgb.g as f32 / 255.0,
                rgb.b as f32 / 255.0,
                1.0,
            ],
            _ => [1.0, 1.0, 1.0, 1.0],
        };

        //
        // Build the six faces of the cube
        //
        for face_index in 0..6 {
            let normal = face_normals[face_index];

            // Can skip the face if the voxel in the direction of the normal is
            // verifiably solid
            let neighbor = offset + normal;
            if !voxel_set.is_empty_f32(neighbor.x, neighbor.y, neighbor.z) {
                continue;
            }

            let mut face_color = Vec3::new(0.0, 0.0, 0.0);
            face_color = Vec3::new(rgba[0], rgba[1], rgba[2]);

            let mut face_uvs = Vec::new();
            match face_index {
                0 => {
                    for p in tri_points.iter() {
                        let q = Vec3::new(p.x, p.y, 0.0);
                        positions.push((q + offset).into());
                        face_uvs.push(Vec2::new(p.x, 1.0 - p.y));
                    }
                }
                1 => {
                    for p in tri_points.iter().rev() {
                        let q = Vec3::new(p.x, p.y, 1.0);
                        positions.push((q + offset).into());
                        face_uvs.push(Vec2::new(p.x, 1.0 - p.y));
                    }
                }
                2 => {
                    for p in tri_points.iter() {
                        let q = Vec3::new(0.0, p.x, p.y);
                        positions.push((q + offset).into());
                        face_uvs.push(Vec2::new(1.0 - p.x, 1.0 - p.y));
                    }
                }
                3 => {
                    for p in tri_points.iter().rev() {
                        let q = Vec3::new(1.0, p.x, p.y);
                        positions.push((q + offset).into());
                        face_uvs.push(Vec2::new(p.x, 1.0 - p.y));
                    }
                }
                4 => {
                    for p in tri_points.iter() {
                        let q = Vec3::new(p.y, 0.0, p.x);
                        positions.push((q + offset).into());
                        face_uvs.push(Vec2::new(p.y, 1.0 - p.x));
                    }
                }
                5 => {
                    for p in tri_points.iter().rev() {
                        let q = Vec3::new(p.y, 1.0, p.x);
                        positions.push((q + offset).into());
                        face_uvs.push(Vec2::new(1.0 - p.y, 1.0 - p.x));
                    }
                }
                _ => {
                    continue;
                }
            };

            for _ in 0..tri_points.len() {
                normals.push(normal.into());
                colors.push([face_color.x, face_color.y, face_color.z, 1.0]);
            }
        }
    }

    positions.shrink_to_fit();
    normals.shrink_to_fit();
    colors.shrink_to_fit();

    VoxelMesh {
        positions,
        normals,
        colors,
    }
}
