mod internal {
    pub use bevy::core::FrameCount;
    pub use bevy::prelude::*;
    pub use snowfall_bevy::prelude::*;
    pub use snowfall_voxel::prelude::*;
}

use crate::internal::*;

use clap::Parser;

#[derive(Parser)]
struct CLIArguments {
    /// Name of the file to load
    filename: String,
}

#[derive(Resource)]
struct AppState {
    filename: String,

    file_modification: u64,
    view_radius: f32,
    look_at: Vec3,
}

impl AppState {
    fn new(filename: String) -> Self {
        Self {
            filename,
            file_modification: 0,
            view_radius: 32.0,
            look_at: Vec3::new(0.0, 0.0, 1.0),
        }
    }
}

fn main() {
    let args = CLIArguments::parse();

    let mut app = App::new();
    app //
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            level: bevy::log::Level::WARN,
            ..default()
        }))
        .insert_resource(AppState::new(args.filename))
        .add_systems(
            Startup,
            (
                startup, //
                startup_init_model.after(startup),
            ),
        )
        .add_systems(
            Update,
            (
                update_model,
                update_camera_rotation.after(update_model), //
            ),
        )
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
            illuminance: light_consts::lux::FULL_DAYLIGHT * 0.5,
            color: Color::srgb(1.0, 1.0, 0.9),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(32.0, 16.0, 32.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Z),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::FULL_DAYLIGHT * 0.15,
            color: Color::srgb(0.9, 0.9, 1.0),
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(-24.0, -16.0, 64.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Z),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(256.0, 256.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.25, 0.25, 0.25))),
        Transform::from_rotation(Quat::from_rotation_x(FRAC_PI_2)),
    ));
}

fn startup_init_model(state: Res<AppState>) {
    let filename = state.filename.clone();

    // Get the basename
    let basename = std::path::Path::new(&filename)
        .file_stem()
        .expect("Failed to get basename")
        .to_str()
        .expect("Failed to convert to string")
        .to_string();

    // Split the extension
    let mut parts = basename.split('-').collect::<Vec<&str>>().clone();
    let seed = parts
        .pop()
        .expect("Failed to get seed")
        .parse::<u64>()
        .unwrap();
    let model = parts.pop().expect("Failed to get model");

    regenerate_model(model.to_string(), seed);
}

fn regenerate_model(model: String, seed: u64) {
    // Run the process model_generator with the args model and seed
    let output = std::process::Command::new("cargo")
        .current_dir("../model_generator")
        .args(&["run", "--release", "--", &model, &seed.to_string()])
        .output()
        .expect("Failed to run model_generator");
    if !output.status.success() {
        error!("Failed to run model_generator");
        println!("{:#?}", output.stdout);
        println!("{:#?}", output.stderr);
    } else {
        println!("Generated model: {}-{}", model, seed);
    }
}

fn update_camera_rotation(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    state: Res<AppState>,
    frame_count: Res<FrameCount>,
) {
    let angle = frame_count.0 as f32 * -0.005;
    let angle_z = frame_count.0 as f32 * 0.005;
    let x = state.view_radius * angle.cos() + state.look_at.x;
    let y = state.view_radius * angle.sin() + state.look_at.y;
    let z = state.view_radius / 3.0 * angle_z.sin() + state.view_radius / 2.0 + state.look_at.z;

    let mut transform = camera_query.single_mut();
    *transform = Transform::from_xyz(x, y, z).looking_at(state.look_at, Vec3::Z);
}

fn update_model(
    mut commands: Commands, //
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut state: ResMut<AppState>,
    mut voxel_query: Query<&mut VoxelMeshComponent>,
    frame_count: Res<FrameCount>,
) {
    if frame_count.0 % 20 != 0 {
        return;
    }

    use std::time::UNIX_EPOCH;

    let filename = state.filename.clone();

    // Get the file timestamp
    let metadata = std::fs::metadata(&filename).unwrap();
    let modified = metadata.modified().unwrap();
    let modified = modified.duration_since(UNIX_EPOCH).unwrap().as_secs();
    if modified <= state.file_modification {
        return;
    }
    state.file_modification = modified;

    for mut entity in voxel_query.iter_mut() {
        println!("Removing entity: {:?}", entity);
        entity.despawn(&mut commands, &mut meshes, &mut materials);
    }

    // Get extension of filename
    let extension = filename
        .split('.')
        .last()
        .expect("Failed to get extension")
        .to_lowercase();
    match extension.as_str() {
        "yaml" => {
            let file: VoxelSceneFile =
                serde_yaml::from_str(&std::fs::read_to_string(&filename).unwrap()).unwrap();

            let mut min = VSVec3::new(i32::MAX, i32::MAX, i32::MAX);
            let mut max = VSVec3::new(i32::MIN, i32::MIN, i32::MIN);
            for layer in &file.scene.layers {
                for object in &layer.objects {
                    println!("{}", &object.model_id);
                    regenerate_model(object.model_id.clone(), object.seed);

                    // TODO: this flat-out assumes it's a .bin file
                    let filename = format!(
                        "../model_generator/content/{}-{}.bin",
                        object.model_id, object.seed
                    );
                    let Ok(model) = VoxelSet::deserialize_from_file(&filename) else {
                        error!("Failed to deserialize voxel set");
                        return;
                    };

                    let bounds = model.bounds();
                    min.x = min.x.min(bounds.0.x);
                    min.y = min.y.min(bounds.0.y);
                    min.z = min.z.min(bounds.0.z);
                    max.x = max.x.max(bounds.1.x);
                    max.y = max.y.max(bounds.1.y);
                    max.z = max.z.max(bounds.1.z);

                    VoxelMeshComponent::spawn_from_model(
                        &model,
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        Vec3::new(
                            object.position.x as f32,
                            object.position.y as f32,
                            object.position.z as f32,
                        ),
                    );
                }
            }

            let bounds = (min, max);
            let max_extent = (((bounds.1.x - bounds.0.x + 1).pow(2)
                + (bounds.1.y - bounds.0.y + 1).pow(2)
                + (bounds.1.z - bounds.0.z + 1).pow(2)) as f32)
                .sqrt();
            let center_point = VSVec3::midpoint(&bounds.0, &bounds.1).to_ws();

            state.look_at = center_point.into();
            state.view_radius = max_extent as f32 * 0.5;
        }
        "bin" => {
            let Ok(model) = VoxelSet::deserialize_from_file(&filename) else {
                error!("Failed to deserialize voxel set");
                return;
            };

            let bounds = model.bounds();
            let max_extent = (bounds.1.x - bounds.0.x + 1)
                .max(bounds.1.y - bounds.0.y + 1)
                .max(bounds.1.z - bounds.0.z + 1);
            let center_point = VSVec3::midpoint(&bounds.0, &bounds.1).to_ws();

            state.look_at = center_point.into();
            state.view_radius = max_extent as f32 * 1.5;

            VoxelMeshComponent::spawn_from_model(
                &model,
                &mut commands,
                &mut meshes,
                &mut materials,
                Vec3::new(0.0, 0.0, 0.0),
            );
        }
        _ => {
            error!("Unknown file extension: {}", extension);
            std::process::exit(1);
        }
    };
}
