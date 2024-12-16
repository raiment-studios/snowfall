use crate::internal::*;

pub struct ChunkFull {
    data: [u8; CHUNK_SIZE],
    palette: ChunkPalette,
}

impl ChunkFull {
    pub fn new() -> Self {
        Self {
            data: [0; CHUNK_SIZE],
            palette: ChunkPalette::new(),
        }
    }
    pub fn set(&mut self, p: (u8, u8, u8), block_index: usize) {
        let local = self.palette.to_local(block_index);
        let index = self.index(p);
        self.data[index] = local;
    }

    pub fn index(&self, p: (u8, u8, u8)) -> usize {
        // Z is kept contiguous with the assumption there are
        // more linear searches across Z than X or Y
        p.2 as usize + p.0 as usize * CHUNK_DIM_Z + p.1 as usize * CHUNK_DIM_X * CHUNK_DIM_Z
    }

    /// Returns true if and only if the entire chunks contains nothing
    /// by index 0 (fully empty) voxels.
    pub fn is_chunk_empty(&self) -> bool {
        for i in 0..CHUNK_SIZE {
            if self.data[i] != 0 {
                return false;
            }
        }
        true
    }

    pub fn is_voxel_empty(&self, p: (u8, u8, u8)) -> bool {
        self.data[self.index(p)] == 0
    }

    pub fn get_voxel(&self, p: (u8, u8, u8)) -> usize {
        self.palette.to_global(self.data[self.index(p)])
    }
}
