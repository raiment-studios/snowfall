use crate::internal::*;

// ============================================================================
// Chunk
// ============================================================================

pub enum Chunk {
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

// ============================================================================
// Functions
// ============================================================================

pub fn chunk_coords(p: IVec3) -> (IVec3, (u8, u8, u8)) {
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
