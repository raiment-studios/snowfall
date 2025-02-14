mod start;

mod internal {
    pub use bevy::prelude::*;
    pub use snowfall_bevy::prelude::*;
    pub use snowfall_voxel::prelude::*;
}

fn main() {
    start::run();
}
