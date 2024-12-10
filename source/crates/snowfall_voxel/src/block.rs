use crate::internal::*;

/// A Block is a definition of a "type" of voxel, where as voxel is a
/// specific instance of a block in a model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: String, // unique identifier for the block (e.g. "grass", "sand")
    pub shader: BlockShader,
}

impl Block {
    pub fn empty() -> Self {
        Block {
            id: "empty".to_string(),
            shader: BlockShader::Empty,
        }
    }

    pub fn color<T>(id: T, r: u8, g: u8, b: u8) -> Self
    where
        T: Into<String>,
    {
        Block {
            id: id.into(),
            shader: BlockShader::RGB(BlockRGB { r, g, b }),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.shader {
            BlockShader::Empty => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockShader {
    Empty,
    RGB(BlockRGB),
}
