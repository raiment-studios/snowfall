use crate::internal::*;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct PaletteIndex(u16);

impl PaletteIndex {
    pub fn zero() -> Self {
        PaletteIndex(0)
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

/// PaletteIndexAlias is a trait to allow VoxelSet and other structs to accept
/// **either** palette indices or strings as block identifiers.
///
/// Using an index directly is almost always going to be more efficient, but
/// for non-performance sensitive / prototype code, being able to refer to
/// blocks by ID is very convenient.
///
pub trait PaletteIndexAlias {
    fn as_index(&self, palette: &VoxelPalette) -> PaletteIndex;
}

impl PaletteIndexAlias for PaletteIndex {
    fn as_index(&self, _palette: &VoxelPalette) -> PaletteIndex {
        *self
    }
}

impl PaletteIndexAlias for &str {
    fn as_index(&self, palette: &VoxelPalette) -> PaletteIndex {
        palette.index_for_id(self)
    }
}

impl PaletteIndexAlias for String {
    fn as_index(&self, palette: &VoxelPalette) -> PaletteIndex {
        palette.index_for_id(self)
    }
}

/// By definition, index 0 is **always** an empty block.
///
#[derive(Serialize, Deserialize, Clone)]
pub struct VoxelPalette {
    blocks: Vec<Block>,
}

impl VoxelPalette {
    pub fn new() -> Self {
        let mut blocks = Vec::new();
        blocks.push(Block::empty());
        VoxelPalette { blocks }
    }

    /// Adds the block to the palette for this voxel set.  If a block with the
    /// same name already exists, it will **replace** that definition.
    pub fn register(&mut self, block: Block) -> PaletteIndex {
        let index = if let Some(index) = self.blocks.iter().position(|b| b.id == block.id) {
            self.blocks[index] = block;
            index
        } else {
            self.blocks.push(block);
            self.blocks.len() - 1
        };
        PaletteIndex(index as u16)
    }

    /// Adds the block to the palette if there is not already such a block.
    pub fn ensure(&mut self, block: Block) -> PaletteIndex {
        let i = if let Some(index) = self.blocks.iter().position(|b| b.is_equivalent(&block)) {
            index as u16
        } else {
            self.blocks.push(block);
            (self.blocks.len() - 1) as u16
        };
        PaletteIndex(i)
    }

    pub fn get(&self, index: PaletteIndex) -> Option<&Block> {
        self.blocks.get(index.0 as usize)
    }

    pub fn index_for_id(&self, id: &str) -> PaletteIndex {
        match self
            .blocks
            .iter()
            .position(|b| b.id == id)
            .map(|i| PaletteIndex(i as u16))
        {
            Some(index) => index,
            None => PaletteIndex::zero(),
        }
    }
}
