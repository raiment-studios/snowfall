use crate::internal::*;

pub struct ChunkSparse {
    data: HashMap<(u8, u8), HashMap<u8, u8>>,
    palette: ChunkPalette,
}

impl ChunkSparse {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            palette: ChunkPalette::new(),
        }
    }

    pub fn set(&mut self, p: (u8, u8, u8), block_index: usize) {
        let local = self.palette.to_local(block_index);
        let inner = self.data.entry((p.0, p.1)).or_insert_with(HashMap::new);
        inner.insert(p.2, local);
    }

    pub fn is_voxel_empty(&self, p: (u8, u8, u8)) -> bool {
        let local = self.get_local(p);
        local == 0
    }

    pub fn get_local(&self, p: (u8, u8, u8)) -> u8 {
        if let Some(inner) = self.data.get(&(p.0, p.1)) {
            if let Some(local) = inner.get(&p.2) {
                return *local;
            }
        }
        0
    }

    pub fn get_voxel(&self, p: (u8, u8, u8)) -> usize {
        let local = self.get_local(p);
        if local == 0 {
            0
        } else {
            self.palette.to_global(local)
        }
    }
}
