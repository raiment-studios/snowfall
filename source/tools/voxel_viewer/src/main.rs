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
    generator: String,
    seed: u64,
}

#[derive(Resource)]
struct AppState {
    generator: String,
    seed: u64,
    view_radius: f32,
    look_at: Vec3,
}

impl AppState {
    fn new(generator: String, seed: u64) -> Self {
        Self {
            generator,
            seed,
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
        .insert_resource(AppState::new(args.generator, args.seed))
        .add_systems(
            Startup,
            (
                startup, //
            ),
        )
        .add_systems(
            Update,
            (
                update_camera_rotation, //
            ),
        )
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut state: ResMut<AppState>,
) {
    use std::f32::consts::FRAC_PI_2;

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
        Transform::from_rotation(Quat::from_rotation_x(FRAC_PI_2))
            .with_translation(Vec3::new(0.0, 0.0, 1.0)),
    ));

    // ------------------------------------------------------------------------

    let mut scene = Scene { models: Vec::new() };
    generate(
        state.generator.as_str(),
        state.seed,
        IVec3::ZERO,
        serde_json::Value::Null,
        &mut scene,
    );

    let mut scene_bounds = (
        IVec3::new(i32::MAX, i32::MAX, i32::MAX),
        IVec3::new(i32::MIN, i32::MIN, i32::MIN),
    );
    for model in scene.models {
        let position = model.position.clone();
        let ModelType::VoxelSet(model) = model.model else {
            continue;
        };
        let bounds = model.bounds();
        scene_bounds.0.x = scene_bounds.0.x.min(bounds.0.x);
        scene_bounds.0.y = scene_bounds.0.y.min(bounds.0.y);
        scene_bounds.0.z = scene_bounds.0.z.min(bounds.0.z);
        scene_bounds.1.x = scene_bounds.1.x.max(bounds.1.x);
        scene_bounds.1.y = scene_bounds.1.y.max(bounds.1.y);
        scene_bounds.1.z = scene_bounds.1.z.max(bounds.1.z);

        VoxelMeshComponent::spawn_from_model(
            &model,
            &mut commands,
            &mut meshes,
            &mut materials,
            Vec3::new(position.x as f32, position.y as f32, position.z as f32),
        );
    }

    let bounds = scene_bounds;
    let max_extent = (((bounds.1.x - bounds.0.x + 1).pow(2)
        + (bounds.1.y - bounds.0.y + 1).pow(2)
        + (bounds.1.z - 0 + 1).pow(2)) as f32)
        .sqrt();
    let center_point = Vec3::new(
        (bounds.0.x + bounds.1.x) as f32 / 2.0,
        (bounds.0.y + bounds.1.y) as f32 / 2.0,
        (bounds.0.z + bounds.1.z) as f32 / 2.0,
    );

    state.look_at = center_point.into();
    state.view_radius = max_extent as f32 * 0.5;
}

struct Scene {
    models: Vec<Model>,
}

fn generate(
    generator: &str,
    seed: u64,
    center: IVec3,
    params: serde_json::Value,
    scene: &mut Scene,
) {
    let mut ctx = GenContext::new();
    ctx.center = center;
    for i in 0..scene.models.len() {
        ctx.ground_objects.push(&scene.models[i]);
    }
    let filename = format!("content/{}-{}.yaml", generator, seed);

    // Check if filename exists
    let model = if std::path::Path::new(&filename).exists() {
        println!("Loading model from file: {}", &filename);
        let contents = std::fs::read_to_string(&filename).unwrap();
        let file: VoxelSceneFile = serde_yaml::from_str(&contents).unwrap();
        ModelType::VoxelScene(Box::new(file.scene))
    } else {
        println!("Generating model: {}", &filename);
        generate_model(generator, seed, params, &ctx)
    };

    match &model {
        ModelType::Empty => {}
        ModelType::VoxelSet(_) => {}
        ModelType::VoxelScene(model) => {
            for layer in &model.layers {
                for object in &layer.objects {
                    println!("{} {:#?}", &object.model_id, &object.params);
                    generate(
                        object.model_id.clone().as_str(),
                        object.seed,
                        object.position,
                        object.params.clone(),
                        scene,
                    );
                }
            }
        }
    };

    scene.models.push(Model {
        model: model,
        position: center,
    });
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
