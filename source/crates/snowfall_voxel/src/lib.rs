mod block;
mod generators;
mod paint;
mod point_set;
mod scene2;
mod voxel_grid;
mod voxel_model;
mod voxel_scene;
mod voxel_set;

pub mod prelude {
    // TODO: tidy up the wildcard exports once this crate stabilizes a bit
    pub use crate::block::*;
    pub mod generators {
        pub use crate::generators::*;
    }
    pub use super::generators::generate_model;
    pub use crate::paint::{GenContext, Model};
    pub use crate::point_set::*;
    pub use crate::scene2::*;
    pub use crate::voxel_grid::{VoxelGrid, VoxelGridGenerator, VoxelGridPager};
    pub use crate::voxel_model::*;
    pub use crate::voxel_scene::*;
    pub use crate::voxel_set::*;
}

mod internal {
    pub use bevy_math::prelude::*;
    pub use serde::{Deserialize, Serialize};
    pub use std::collections::HashMap;

    pub use snowfall_core::prelude::*;

    pub use super::paint::*;
    pub use crate::prelude::*;
    pub use crate::voxel_grid::*;
    pub mod generators {
        pub use super::super::generators::*;
    }

    pub fn from_ws(x: f32, y: f32, z: f32) -> IVec3 {
        IVec3::new(x.floor() as i32, y.floor() as i32, z.floor() as i32)
    }
}
