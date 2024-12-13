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

    let mut rng = snowfall_core::prelude::RNG::new(state.seed + 23849);

    let mut models = Vec::new();

    for j in 0..8 {
        println!("Adding model {}", j);
        let mut ctx = GenContext {
            center: IVec3::new(0, 0, 0),
            ground_objects: vec![],
        };
        for i in 0..models.len() {
            ctx.ground_objects.push(&models[i]);
        }

        const R: i32 = 128;
        let seed = rng.range(1..8192);
        let x: i32 = rng.sign() * rng.range(0..=R);
        let y: i32 = rng.sign() * rng.range(0..=R);
        ctx.center = IVec3::new(x, y, 0);
        let hill: ModelType = generate_small_hill(seed, &ctx).into();
        let model = Model {
            model: hill,
            position: ctx.center.clone(),
        };
        models.push(model);
    }

    let mut ctx = GenContext {
        center: IVec3::new(0, 0, 0),
        ground_objects: vec![],
    };
    for i in 0..models.len() {
        ctx.ground_objects.push(&models[i]);
    }

    match generate_model(state.generator.as_str(), state.seed, &ctx) {
        ModelType::VoxelSet(model) => {
            let bounds = model.bounds();
            let max_extent = (bounds.1.x - bounds.0.x + 1)
                .max(bounds.1.y - bounds.0.y + 1)
                .max(bounds.1.z - bounds.0.z + 1);
            let center_point = VSVec3::midpoint(&bounds.0, &bounds.1).to_ws();

            state.look_at = center_point.into();
            state.view_radius = (max_extent as f32 * 1.15).max(8.0);

            VoxelMeshComponent::spawn_from_model(
                &model,
                &mut commands,
                &mut meshes,
                &mut materials,
                Vec3::new(0.0, 0.0, 0.0),
            );
        }
        ModelType::VoxelScene(model) => {
            let mut min = VSVec3::new(i32::MAX, i32::MAX, i32::MAX);
            let mut max = VSVec3::new(i32::MIN, i32::MIN, i32::MIN);
            for layer in &model.layers {
                for object in &layer.objects {
                    println!("{}", &object.model_id);
                    let model = generate_model(object.model_id.clone().as_str(), object.seed, &ctx);
                    let model = match model {
                        ModelType::VoxelSet(model) => model,
                        ModelType::VoxelScene(_) => {
                            eprintln!("VoxelScene objects are not supported");
                            std::process::exit(1);
                        }
                        ModelType::Empty => {
                            eprintln!("Unknown generator: {}", object.model_id);
                            std::process::exit(1);
                        }
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
                + (bounds.1.z - 0 + 1).pow(2)) as f32)
                .sqrt();
            let center_point = VSVec3::midpoint(&bounds.0, &bounds.1).to_ws();

            state.look_at = center_point.into();
            state.view_radius = max_extent as f32 * 2.5;
        }
        _ => {
            eprintln!("Unknown generator: {}", state.generator);
            std::process::exit(1);
        }
    }

    for _i in 0..32 {
        const R: i32 = 96;
        let seed = rng.range(1..8192);
        let x: i32 = rng.range(-R..=R);
        let y: i32 = rng.range(-R..=R);
        ctx.center = IVec3::new(x, y, 0);
        let model = generate_pine_tree(seed, &ctx);
        VoxelMeshComponent::spawn_from_model(
            &model,
            &mut commands,
            &mut meshes,
            &mut materials,
            Vec3::new(x as f32, y as f32, 0.0),
        );
    }

    for model in ctx.ground_objects.iter() {
        let p = model.position.clone();
        let ModelType::VoxelSet(voxel_set) = &model.model else {
            panic!("Expected VoxelSet");
        };
        VoxelMeshComponent::spawn_from_model(
            voxel_set,
            &mut commands,
            &mut meshes,
            &mut materials,
            Vec3::new(p.x as f32, p.y as f32, p.z as f32),
        );
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
