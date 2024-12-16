use crate::internal::*;

pub struct Palette {
    blocks: Vec<Block>,
    block_index: HashMap<String, usize>,
}

impl Palette {
    pub fn new() -> Self {
        Self {
            blocks: vec![Block::empty()],
            block_index: HashMap::new(),
        }
    }

    pub fn register(&mut self, block: Block) {
        let index = self.blocks.len();
        self.blocks.push(block.clone());
        self.block_index.insert(block.id.clone(), index);
    }

    pub fn index_by_id(&self, id: &str) -> Option<usize> {
        self.block_index.get(id).copied()
    }

    pub fn block_by_id(&self, id: &str) -> Option<&Block> {
        self.block_index.get(id).map(|&index| &self.blocks[index])
    }

    pub fn block_by_index(&self, index: usize) -> Option<&Block> {
        self.blocks.get(index)
    }
}
