mod block;
mod voxel_grid;
mod voxel_set;
mod vs_vec3;

pub mod prelude {
    // TODO: tidy up the wildcard exports once this crate stabilizes a bit
    pub use crate::block::*;
    pub use crate::voxel_grid::*;
    pub use crate::voxel_set::*;
    pub use crate::vs_vec3::*;
}

mod internal {
    pub use std::collections::HashMap;

    pub use crate::prelude::*;

    pub use super::block::*;
    pub use super::voxel_grid::*;
    pub use super::voxel_set::*;
    pub use super::vs_vec3::*;
}