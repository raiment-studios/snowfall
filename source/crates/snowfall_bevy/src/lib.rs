pub mod prelude {

    pub use crate::NaiveVoxelComponent;
}

mod internal {
    pub use crate::prelude::*;

    pub use bevy::prelude::*;
    pub use std::collections::HashMap;

    pub use snowfall_voxel::prelude::*;
}

use crate::internal::*;

/// NaiveVoxelComponent is an intentionally simple Bevy component for rendering
/// a mesh mesh.  It is **not** efficient and is intended to be as simple as
/// possible for learning, demonstration, or prototyping purposes.
///
#[derive(Component, Debug)]
pub struct NaiveVoxelComponent {
    pub parent_id: Entity,
    pub cube_mesh: Handle<Mesh>,
    pub materials: HashMap<String, Handle<StandardMaterial>>,
}

impl NaiveVoxelComponent {
    pub fn spawn_from_model(
        model: &VoxelSet,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        // The code creates a separate entity for each voxel.  We cluster these
        // under a parent entity for ease of removal and transformation.
        let parent = commands.spawn(Transform::from_xyz(0.0, 0.0, 0.0)).id();

        // Create a single geometry object for all the voxels. Only the transformation
        // will differ.  Note that cube.clone() is not a deep clone and reusing the
        // same object is more efficient.
        let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));

        // We build a cache to reuse materials for each block type in the model.
        let mut cache = HashMap::<String, Handle<StandardMaterial>>::new();

        // Now iterate over all the non-empty voxels and create a child entity for each
        for (VSVec3 { x, y, z }, block) in model.voxel_iter(false) {
            let material = cache.entry(block.id.to_string()).or_insert_with(|| {
                let (r, g, b) = match block.shader {
                    BlockShader::Empty => panic!("Should have been filtered out!"),
                    BlockShader::RGB(ref rgb) => rgb.clone(),
                }
                .to_srgb();
                materials.add(Color::srgb(r, g, b))
            });

            let child = commands
                .spawn((
                    Mesh3d(cube.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::from_xyz(x as f32 + 0.5, y as f32 + 0.5, z as f32 + 0.5),
                ))
                .id();
            commands.entity(parent).add_child(child);
        }

        // Insert the parent component and record the fields needed to clean
        // up the component properly.
        commands.entity(parent).insert(NaiveVoxelComponent {
            parent_id: parent,
            cube_mesh: cube,
            materials: cache,
        });
    }

    pub fn despawn(
        &mut self,
        commands: &mut Commands, //
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        commands.entity(self.parent_id).despawn_recursive();

        meshes.remove(&self.cube_mesh);
        for handle in self.materials.values() {
            materials.remove(handle);
        }
    }
}
