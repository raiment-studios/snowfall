use crate::internal::*;

pub const CHUNK_DIM_X: usize = 8;
pub const CHUNK_DIM_Y: usize = 8;
pub const CHUNK_DIM_Z: usize = 16;
pub const CHUNK_SIZE: usize = CHUNK_DIM_X * CHUNK_DIM_Y * CHUNK_DIM_Z;

/// Interface for allowing chunks to be paged in / out of memory.
pub trait VoxelGridPager {
    fn read_chunk(&self, p: IVec3) -> Option<Chunk>;
    fn write_chunk(&self, p: IVec3, chunk: Chunk);
}

/// Interface for providing the contents of a new chunk.
pub trait VoxelGridGenerator {
    fn generate(&self, world_position: IVec3) -> &str;
}

/// VoxelGrid is a 3D grid of voxels designed for handling unbounded, sparse
/// voxel data.
///
/// As such it is designed with the following in mind:
///
/// - Chunks of the grid can be paged in / out of memory at any time
/// - The grid data structures should be minimal in size
/// - There may be a large number of empty chunks for things like the sky
///
/// It is designed primary for terrain or very large models.
///
pub struct VoxelGrid {
    palette: Palette,
    chunks: HashMap<IVec3, Chunk>,
    pager: Option<Box<dyn VoxelGridPager>>,
    generator: Option<Box<dyn VoxelGridGenerator>>,
}

impl VoxelGrid {
    // ------------------------------------------------------------------------
    // Construction
    // ------------------------------------------------------------------------

    pub fn new() -> Self {
        Self {
            palette: Palette::new(),
            chunks: HashMap::new(),
            pager: None,
            generator: None,
        }
    }

    // ------------------------------------------------------------------------
    // Properties & Utilities
    // ------------------------------------------------------------------------

    // Destructuring Self can avoid lifetime conflicts across use of the
    // different fields. This is intended for internal use.
    fn destructure(
        grid: &mut VoxelGrid,
    ) -> (
        &mut Palette,
        &mut HashMap<IVec3, Chunk>,
        &mut Option<Box<dyn VoxelGridPager>>,
        &mut Option<Box<dyn VoxelGridGenerator>>,
    ) {
        (
            &mut grid.palette,
            &mut grid.chunks,
            &mut grid.pager,
            &mut grid.generator,
        )
    }

    // ------------------------------------------------------------------------
    // Blocks
    // ------------------------------------------------------------------------

    pub fn register_block(&mut self, block: Block) {
        self.palette.register(block);
    }

    // ------------------------------------------------------------------------
    // Voxels
    // ------------------------------------------------------------------------

    /// Note: this method intentionally takes a mutable reference to self
    /// as it can create new chunks if a chunk that's not yet in memory is
    /// queried.
    pub fn is_empty(&mut self, p: IVec3) -> bool {
        let (chunk_pos, inner_pos) = chunk_coords(p.into());
        let chunk = self.ensure_chunk(chunk_pos);
        chunk.is_empty(inner_pos)
    }

    /// Used for mesh generation. If a block is an occluder, it means its six
    /// faces are fully opaque (i.e. neither transparent, empty, nor composed
    /// of sub-voxels that leave portions).
    pub fn is_occuluder(&self, p: IVec3) -> bool {
        todo!()
    }

    pub fn get<S>(&self, p: S) -> Option<&Block>
    where
        S: Into<IVec3>,
    {
        let (chunk_pos, inner_pos) = chunk_coords(p.into());
        if let Some(chunk) = self.chunks.get(&chunk_pos) {
            let block_index = chunk.get(inner_pos);
            return self.palette.block_by_index(block_index);
        }
        None
    }

    pub fn set<S, T>(&mut self, p: S, id: T)
    where
        S: Into<IVec3>,
        T: Into<&'static str>,
    {
        let (chunk_pos, inner_pos) = chunk_coords(p.into());
        let block_index = self.palette.index_by_id(id.into()).unwrap();
        let chunk = self.ensure_chunk(chunk_pos);
        chunk.set(inner_pos, block_index);
    }

    // ------------------------------------------------------------------------
    // Chunks
    // ------------------------------------------------------------------------

    fn ensure_chunk(&mut self, p: IVec3) -> &mut Chunk {
        let (palette, chunks, pager, generator) = Self::destructure(self);

        chunks.entry(p).or_insert_with(|| {
            if let Some(pager) = pager {
                if let Some(chunk) = pager.read_chunk(p) {
                    return chunk;
                }
            }
            if let Some(generator) = &generator {
                return Self::generate_chunk(generator, palette, p);
            }
            return Chunk::Empty;
        })
    }

    fn generate_chunk(
        generator: &Box<dyn VoxelGridGenerator>,
        palette: &mut Palette,
        p: IVec3,
    ) -> Chunk {
        let base = IVec3::new(
            p.x * CHUNK_DIM_X as i32,
            p.y * CHUNK_DIM_Y as i32,
            p.z * CHUNK_DIM_Z as i32,
        );

        let mut chunk = ChunkFull::new();
        for dy in 0..CHUNK_DIM_Y as u8 {
            for dx in 0..CHUNK_DIM_X as u8 {
                for dz in 0..CHUNK_DIM_Z as u8 {
                    let p = base + IVec3::new(dx as i32, dy as i32, dz as i32);
                    let block_name = generator.generate(p);
                    let block_index = palette.index_by_id(block_name).unwrap();
                    chunk.set((dx, dy, dz), block_index);
                }
            }
        }

        // This optimization matters. Don't generate full chunks that contain
        // exclusively empty voxels. At the same time, we don't want the
        // generator to have to figure out if it is going to generate a fully
        // empty chunk since they work a voxel at a time.
        if chunk.is_chunk_empty() {
            Chunk::Empty
        } else {
            Chunk::Full(chunk)
        }
    }
}
