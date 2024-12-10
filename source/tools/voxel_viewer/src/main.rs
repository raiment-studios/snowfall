mod internal {
    pub use bevy::core::FrameCount;
    pub use bevy::prelude::*;
    pub use snowfall_bevy::prelude::*;
    pub use snowfall_voxel::prelude::*;
}

use crate::internal::*;

#[derive(Resource, Default)]
struct AppState {
    file_modification: u64,
}

fn main() {
    let mut app = App::new();
    app //
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            level: bevy::log::Level::WARN,
            ..default()
        }))
        .insert_resource(AppState::default())
        .add_systems(Startup, startup)
        .add_systems(Update, update)
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    use std::f32::consts::{FRAC_PI_2, PI};

    //  Default camera, lights, and ground plane
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(32.0, 16.0, 32.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Z),
        Msaa::Off,
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::FULL_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::YZX, 0.0, PI * -0.15, PI * -0.15)),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
        Transform::from_rotation(Quat::from_rotation_x(FRAC_PI_2)),
    ));
}

fn update(
    mut commands: Commands, //
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut state: ResMut<AppState>,
    mut query: Query<&mut NaiveVoxelComponent>,
    frame_count: Res<FrameCount>,
) {
    if frame_count.0 % 10 != 0 {
        return;
    }

    use std::time::UNIX_EPOCH;

    let path = "../model_generator/content/model.bin";
    // Get the file timestamp
    let metadata = std::fs::metadata(path).unwrap();
    let modified = metadata.modified().unwrap();
    let modified = modified.duration_since(UNIX_EPOCH).unwrap().as_secs();
    if modified <= state.file_modification {
        return;
    }

    for mut entity in query.iter_mut() {
        println!("Removing entity: {:?}", entity);
        entity.despawn(&mut commands, &mut meshes, &mut materials);
    }

    let model = VoxelSet::deserialize_from_file(path);
    NaiveVoxelComponent::spawn_from_model(&model, commands, meshes, materials);

    state.file_modification = modified;
}
