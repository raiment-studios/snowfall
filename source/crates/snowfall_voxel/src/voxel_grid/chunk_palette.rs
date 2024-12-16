use crate::internal::*;

pub enum ChunkPalette {
    Small { entries: [usize; 16] },
    Full(Vec<usize>),
}

impl ChunkPalette {
    pub fn new() -> Self {
        Self::Small { entries: [0; 16] }
    }
    pub fn to_local(&mut self, block_index: usize) -> u8 {
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

    pub fn to_global(&self, local_index: u8) -> usize {
        match self {
            Self::Small { entries } => entries[local_index as usize],
            Self::Full(entries) => entries[local_index as usize],
        }
    }
}
