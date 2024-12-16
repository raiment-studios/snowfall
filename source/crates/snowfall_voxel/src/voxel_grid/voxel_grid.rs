use crate::internal::*;

const CHUNK_DIM_X: usize = 8;
const CHUNK_DIM_Y: usize = 8;
const CHUNK_DIM_Z: usize = 16;
const CHUNK_SIZE: usize = CHUNK_DIM_X * CHUNK_DIM_Y * CHUNK_DIM_Z;

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
            return Some(&self.palette.blocks[block_index]);
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

fn chunk_coords(p: IVec3) -> (IVec3, (u8, u8, u8)) {
    let outer = IVec3::new(
        p.x.div_euclid(CHUNK_DIM_X as i32),
        p.y.div_euclid(CHUNK_DIM_Y as i32),
        p.z.div_euclid(CHUNK_DIM_Z as i32),
    );
    let inner = (
        p.x.rem_euclid(CHUNK_DIM_X as i32) as u8,
        p.y.rem_euclid(CHUNK_DIM_Y as i32) as u8,
        p.z.rem_euclid(CHUNK_DIM_Z as i32) as u8,
    );
    (outer, inner)
}

struct Palette {
    blocks: Vec<Block>,
    block_index: HashMap<String, usize>,
}

impl Palette {
    fn new() -> Self {
        Self {
            blocks: vec![Block::empty()],
            block_index: HashMap::new(),
        }
    }

    fn register(&mut self, block: Block) {
        let index = self.blocks.len();
        self.blocks.push(block.clone());
        self.block_index.insert(block.id.clone(), index);
    }

    fn index_by_id(&self, id: &str) -> Option<usize> {
        self.block_index.get(id).copied()
    }

    fn by_id(&self, id: &str) -> Option<&Block> {
        self.block_index.get(id).map(|&index| &self.blocks[index])
    }
}

enum Chunk {
    Empty,
    Full(ChunkFull),
    Sparse(ChunkSparse),
}

impl Chunk {
    pub fn is_empty(&self, p: (u8, u8, u8)) -> bool {
        match self {
            Chunk::Empty => true,
            Chunk::Full(imp) => imp.is_voxel_empty(p),
            Chunk::Sparse(imp) => imp.is_voxel_empty(p),
        }
    }

    pub fn get(&self, p: (u8, u8, u8)) -> usize {
        match self {
            Chunk::Empty => 0,
            Chunk::Full(imp) => imp.get_voxel(p),
            Chunk::Sparse(imp) => imp.get_voxel(p),
        }
    }

    pub fn set(&mut self, p: (u8, u8, u8), block_index: usize) {
        match self {
            Chunk::Empty => {
                let mut imp = ChunkFull::new();
                imp.set(p, block_index);
                *self = Chunk::Full(imp);
            }
            Chunk::Full(imp) => imp.set(p, block_index),
            Chunk::Sparse(imp) => imp.set(p, block_index),
        };
    }
}

enum ChunkPalette {
    Small { entries: [usize; 16] },
    Full(Vec<usize>),
}

impl ChunkPalette {
    fn new() -> Self {
        Self::Small { entries: [0; 16] }
    }
    fn to_local(&mut self, block_index: usize) -> u8 {
        match self {
            Self::Small { entries } => {
                for i in 0..16 {
                    if entries[i] == block_index {
                        return i as u8;
                    }
                }
                for i in 0..16 {
                    if entries[i] == 0 {
                        entries[i] = block_index;
                        return i as u8;
                    }
                }
                let mut vec = entries.to_vec();
                let i = vec.len();
                vec.push(block_index);
                *self = Self::Full(vec);
                i as u8
            }
            Self::Full(entries) => {
                for i in 0..entries.len() {
                    if entries[i] == block_index {
                        return i as u8;
                    }
                }
                let i = entries.len();
                entries.push(block_index);
                return i as u8;
            }
        }
    }

    fn to_global(&self, local_index: u8) -> usize {
        match self {
            Self::Small { entries } => entries[local_index as usize],
            Self::Full(entries) => entries[local_index as usize],
        }
    }
}

struct ChunkFull {
    data: [u8; CHUNK_SIZE],
    palette: ChunkPalette,
}

impl ChunkFull {
    fn new() -> Self {
        Self {
            data: [0; CHUNK_SIZE],
            palette: ChunkPalette::new(),
        }
    }
    fn set(&mut self, p: (u8, u8, u8), block_index: usize) {
        let local = self.palette.to_local(block_index);
        self.data[p.0 as usize
            + p.1 as usize * CHUNK_DIM_X
            + p.2 as usize * CHUNK_DIM_X * CHUNK_DIM_Y] = local;
    }

    fn index(&self, p: (u8, u8, u8)) -> usize {
        // Z is kept contiguous with the assumption there are
        // more linear searches across Z than X or Y
        p.2 as usize + p.0 as usize * CHUNK_DIM_Z + p.1 as usize * CHUNK_DIM_X * CHUNK_DIM_Z
    }

    /// Returns true if and only if the entire chunks contains nothing
    /// by index 0 (fully empty) voxels.
    fn is_chunk_empty(&self) -> bool {
        for i in 0..CHUNK_SIZE {
            if self.data[i] != 0 {
                return false;
            }
        }
        true
    }

    fn is_voxel_empty(&self, p: (u8, u8, u8)) -> bool {
        self.data[self.index(p)] == 0
    }

    fn get_voxel(&self, p: (u8, u8, u8)) -> usize {
        self.palette.to_global(self.data[self.index(p)])
    }
}

struct ChunkSparse {
    data: HashMap<(u8, u8), HashMap<u8, u8>>,
    palette: ChunkPalette,
}

impl ChunkSparse {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
            palette: ChunkPalette::new(),
        }
    }

    fn set(&mut self, p: (u8, u8, u8), block_index: usize) {
        let local = self.palette.to_local(block_index);
        let inner = self.data.entry((p.0, p.1)).or_insert_with(HashMap::new);
        inner.insert(p.2, local);
    }

    fn is_voxel_empty(&self, p: (u8, u8, u8)) -> bool {
        let local = self.get_local(p);
        local == 0
    }

    fn get_local(&self, p: (u8, u8, u8)) -> u8 {
        if let Some(inner) = self.data.get(&(p.0, p.1)) {
            if let Some(local) = inner.get(&p.2) {
                return *local;
            }
        }
        0
    }

    fn get_voxel(&self, p: (u8, u8, u8)) -> usize {
        let local = self.get_local(p);
        if local == 0 {
            0
        } else {
            self.palette.to_global(local)
        }
    }
}
