use crate::internal::*;

/// A Block is a definition of a "type" of voxel, where as voxel is a
/// specific instance of a block in a model.
///
/// Since there is one shared Block for **every** instance, this
/// structure does not need to be as optimized for space.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Block {
    pub id: String, // unique identifier for the block (e.g. "grass", "sand")
    pub shader: BlockShader,

    /// Indicates the voxel cannot be built on top of.
    pub occupied: bool,

    /// Cost should account for a combination of physical difficulty
    /// as well as some subjective preference: i.e. an actor likely
    /// will "want" to walk on a road if one's a couple voxels away.
    /// Thus walking on the road may not be twice as fast but it might
    /// have half the cost to express both the increase speed as well
    /// as the preference.
    ///
    /// Relative to grass:
    /// grass = 1.0
    /// thick underbrush = 5.0
    /// dense rainforest = 20.0
    /// clear road = 0.5
    pub walk_cost: f32,
}

impl Block {
    pub fn empty() -> Self {
        Block {
            id: "empty".to_string(),
            shader: BlockShader::Empty,
            walk_cost: 0.0,
            occupied: false,
        }
    }

    pub fn new<T>(id: T) -> Self
    where
        T: Into<String>,
    {
        Block {
            id: id.into(),
            shader: BlockShader::Empty,
            walk_cost: 1.0,
            occupied: false,
        }
    }

    /// Returns true if all properties other than id match
    pub fn is_equivalent(&self, other: &Block) -> bool {
        self.shader == other.shader
            && self.occupied == other.occupied
            && self.walk_cost == other.walk_cost
    }

    pub fn color<T>(id: T, r: u8, g: u8, b: u8) -> Self
    where
        T: Into<String>,
    {
        let mut block = Block::new(id);
        block.shader = BlockShader::RGB(BlockRGB { r, g, b });
        block
    }

    pub fn with_occupied(&self, occupied: bool) -> Self {
        self.variant(|block| block.occupied = occupied)
    }

    pub fn with_color(&self, r: u8, g: u8, b: u8) -> Self {
        self.variant(|block| block.shader = BlockShader::RGB(BlockRGB { r, g, b }))
    }

    pub fn modify<T>(&self, cb: T) -> Self
    where
        T: Fn(&mut Block),
    {
        let mut block = self.clone();
        cb(&mut block);
        if block.is_equivalent(self) {
            return self.clone();
        }
        // Do **not** update the id
        block
    }

    pub fn variant<T>(&self, cb: T) -> Self
    where
        T: Fn(&mut Block),
    {
        let mut block = self.clone();
        cb(&mut block);
        if block.is_equivalent(self) {
            return self.clone();
        }
        block.id = block.variant_id();
        block
    }

    pub fn variant_id(&self) -> String {
        format!(
            "{}::{}|{}",
            self.id,
            if self.occupied { "X" } else { "O" },
            match self.shader {
                BlockShader::Empty => "E".to_string(),
                BlockShader::RGB(ref rgb) => format!("RGB{},{},{}", rgb.r, rgb.g, rgb.b),
            }
        )
    }

    pub fn is_empty(&self) -> bool {
        match self.shader {
            BlockShader::Empty => true,
            _ => false,
        }
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
