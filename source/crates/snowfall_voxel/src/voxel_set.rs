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
    generation: u64, // Generation number used to track changes
    pub palette: VoxelPalette,

    // Storing the data by z-column is *much* faster in any context where
    // "height at x,y" is a common operation.
    data: HashMap<(i32, i32), HashMap<i32, PaletteIndex>>,
}

impl VoxelSet {
    pub fn new() -> Self {
        VoxelSet {
            generation: 0,
            palette: VoxelPalette::new(),
            data: HashMap::new(),
        }
    }

    // ------------------------------------------------------------------------
    // Block palette
    // ------------------------------------------------------------------------

    pub fn register_block(&mut self, block: Block) -> PaletteIndex {
        self.palette.register(block)
    }

    pub fn ensure_block(&mut self, block: Block) -> PaletteIndex {
        self.palette.ensure(block)
    }

    // ------------------------------------------------------------------------
    // Voxel properties
    // ------------------------------------------------------------------------

    /// Returns the inclusive bounds of the voxel set.
    pub fn bounds(&self) -> IBox3 {
        let mut bounds = IBox3::new();
        for (vc, _) in self.voxel_iter(false) {
            bounds.add(vc);
        }
        bounds
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
            if id.is_zero() {
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

    pub fn top_block_at(&self, x: i32, y: i32) -> Option<&Block> {
        let Some(column) = self.data.get(&(x, y)) else {
            return None;
        };

        let mut height: Option<i32> = None;
        let mut top_block: Option<&Block> = None;
        for (z, id) in column.iter() {
            if id.is_zero() {
                continue;
            }
            if match height {
                Some(h) => h < *z,
                None => true,
            } {
                height = Some(*z);
                top_block = self.palette.get(*id);
            }
        }
        top_block
    }

    // ------------------------------------------------------------------------
    // Voxel manipulation
    // ------------------------------------------------------------------------

    pub fn is_empty(&self, vc: IVec3) -> bool {
        let Some(col) = self.data.get(&(vc.x, vc.y)) else {
            return true;
        };
        match col.get(&vc.z) {
            Some(id) => id.is_zero(),
            None => true,
        }
    }

    pub fn is_empty_f32(&self, x: f32, y: f32, z: f32) -> bool {
        self.is_empty(from_ws(x, y, z))
    }

    pub fn get_voxel<S>(&self, vs: S) -> &Block
    where
        S: Into<IVec3>,
    {
        let vc = vs.into();
        let palette_index = if let Some(column) = self.data.get(&(vc.x, vc.y)) {
            match column.get(&vc.z) {
                Some(id) => *id,
                None => PaletteIndex::zero(),
            }
        } else {
            PaletteIndex::zero()
        };
        self.palette.get(palette_index).unwrap()
    }

    pub fn clear_voxel<S>(&mut self, vc: S)
    where
        S: Into<IVec3>,
    {
        let vc = vc.into();
        let column = self.data.entry((vc.x, vc.y)).or_insert(HashMap::new());
        column.remove(&vc.z);
    }

    pub fn set<P, I>(&mut self, vc: P, id: I)
    where
        P: Into<IVec3>,
        I: PaletteIndexAlias,
    {
        let vc = vc.into();
        let index = id.as_index(&self.palette);
        let column = self.data.entry((vc.x, vc.y)).or_insert(HashMap::new());
        column.insert(vc.z, index);
    }

    pub fn set_voxel<S, T>(&mut self, vc: S, id: T)
    where
        S: Into<IVec3>,
        T: Into<String>,
    {
        let id = id.into();
        let index = self.palette.index_for_id(id.as_str());

        // Get the column or create it
        let vc = vc.into();
        let column = self.data.entry((vc.x, vc.y)).or_insert(HashMap::new());
        column.insert(vc.z, index);
    }

    pub fn modify_voxel<S>(&mut self, vc: S, cb: fn(&Block) -> Block)
    where
        S: Into<IVec3>,
    {
        let vc: IVec3 = vc.into();
        let (index, new_block) = {
            let column = self.data.entry((vc.x, vc.y)).or_insert(HashMap::new());
            let index = *column.get(&vc.z).unwrap_or(&PaletteIndex::zero());
            let block = &self.palette.get(index).unwrap();
            (index, cb(block))
        };
        let new_index = self.ensure_block(new_block);
        if new_index == index {
            return;
        }
        let column = self.data.entry((vc.x, vc.y)).or_insert(HashMap::new());
        column.insert(vc.z, new_index);
    }

    pub fn voxel_iter(&self, include_empty: bool) -> Vec<(IVec3, &Block)> {
        // Collect all the voxels into a vec
        let mut voxels = Vec::new();
        for (x, column) in self.data.iter() {
            for (z, id) in column.iter() {
                let vc = IVec3::new(x.0, x.1, *z);
                let block = self.palette.get(*id).unwrap();
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
    let max_voxel_count = bounds.volume();
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_voxel_set_new() {
        let mut model = VoxelSet::new();
        let index = model.register_block(Block::color("test", 255, 0, 0));

        model.set((0, 0, 0), index);
        model.set((0, 0, 1), "test");

        assert_eq!(model.get_voxel((0, 0, 0)).id, "test");
        assert_eq!(model.get_voxel((0, 0, 1)).id, "test");
        assert_eq!(model.get_voxel((0, 0, 2)).id, "empty");
    }
}
