use crate::internal::*;

/// A Block is a definition of a "type" of voxel, where as voxel is a
/// specific instance of a block in a model.
///
/// Since there is one shared Block for **every** instance, this
/// structure does not need to be as optimized for space.
///
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Block {
    pub id: String, // unique identifier for the block (e.g. "grass", "sand")
    pub shader: BlockShader,

    /// Indicates the voxel cannot be built on top of.
    pub occupied: bool,
}

impl Block {
    pub fn empty() -> Self {
        Block {
            id: "empty".to_string(),
            shader: BlockShader::Empty,
            occupied: false,
        }
    }

    pub fn color<T>(id: T, r: u8, g: u8, b: u8) -> Self
    where
        T: Into<String>,
    {
        Block {
            id: id.into(),
            shader: BlockShader::RGB(BlockRGB { r, g, b }),
            occupied: false,
        }
    }

    pub fn with_occupied(&self, occupied: bool) -> Self {
        let mut block = self.clone();
        if self.occupied == occupied {
            return block;
        }
        block.id = format!("{}&occupied={}", block.id, occupied);
        block.occupied = occupied;
        block
    }

    pub fn is_empty(&self) -> bool {
        match self.shader {
            BlockShader::Empty => true,
            _ => false,
        }
    }

    /// Returns true if all properties other than id match
    pub fn is_equivalent(&self, other: &Block) -> bool {
        self.shader == other.shader && self.occupied == other.occupied
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct BlockRGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl BlockRGB {
    /// Useful for conversion to a Bevy srgb color
    pub fn to_srgb(&self) -> (f32, f32, f32) {
        (
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum BlockShader {
    Empty,
    RGB(BlockRGB),
}
