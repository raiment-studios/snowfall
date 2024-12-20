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
                startup_scene.after(startup),
            ),
        )
        .add_systems(
            Update,
            (
                update_camera_rotation, //
                update_models,
            ),
        )
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
            illuminance: light_consts::lux::FULL_DAYLIGHT * 0.10,
            color: Color::srgb(1.0, 1.0, 0.9),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(32.0, 16.0, 32.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Z),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::FULL_DAYLIGHT * 0.035,
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
            .with_translation(Vec3::new(0.0, 0.0, 0.0)),
    ));
}

fn startup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut state: ResMut<AppState>,
) {
    //
    // Recursively generate the scene from the root generator
    //
    let mut scene = Scene2::new();
    scene.root = generate(
        state.generator.as_str(),
        state.seed,
        IVec3::ZERO,
        serde_json::Value::Null,
        &mut scene,
    );

    //
    // Create the Bevy graphics from the generator models
    //
    let mut scene_bounds = IBox3::new();

    spawn_model(
        &Object {
            generator_id: "".to_string(),
            seed: 0,
            params: serde_json::Value::Null,
            position: IVec3::ZERO,
            scale: 1.0,
            imp: ObjectImp::VoxelSet(Box::new(scene.terrain)),
        },
        &mut scene_bounds,
        &mut commands,
        &mut meshes,
        &mut materials,
    );

    spawn_model(
        &scene.root,
        &mut scene_bounds,
        &mut commands,
        &mut meshes,
        &mut materials,
    );

    let max_extent = ((scene_bounds.length_x().pow(2)
        + scene_bounds.length_y().pow(2)
        + scene_bounds.max.z.pow(2)) as f32)
        .sqrt();

    state.look_at = scene_bounds.center_f32();
    state.view_radius = (max_extent * 0.20).max(20.0);
}

fn spawn_model(
    obj: &Object,
    scene_bounds: &mut IBox3,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    match &obj.imp {
        ObjectImp::Empty => {}
        ObjectImp::Stub => {}
        ObjectImp::Actor(_) => {}
        ObjectImp::VoxelSet(model) => {
            let mut bounds = model.bounds();
            bounds.translate(obj.position);
            scene_bounds.merge(&bounds);

            VoxelMeshComponent::spawn_from_model(
                model,
                commands,
                meshes,
                materials,
                Vec3::new(
                    obj.position.x as f32,
                    obj.position.y as f32,
                    obj.position.z as f32,
                ),
                obj.scale,
            );
        }
        ObjectImp::Group(group) => {
            for object in &group.objects {
                spawn_model(object, scene_bounds, commands, meshes, materials);
            }
        }
    }
}

fn generate(
    generator: &str,
    seed: u64,
    center: IVec3,
    params: serde_json::Value,
    scene: &mut Scene2,
) -> Object {
    let mut ctx = GenContext::new(generator, seed);
    ctx.center = center;
    ctx.params = params.clone();
    let filename = format!("content/{}-{}.yaml", generator, seed);

    // Check if filename exists
    let model = if std::path::Path::new(&filename).exists() {
        println!("Loading model from file: {}", &filename);
        let contents = std::fs::read_to_string(&filename).unwrap();
        let file: VoxelSceneFile = serde_yaml::from_str(&contents).unwrap();
        VoxelModel::VoxelScene(Box::new(file.scene))
    } else {
        println!("Generating model: {}", &filename);
        generate_model(&ctx, scene)
    };

    Object {
        generator_id: generator.to_string(),
        seed,
        params: params.clone(),
        position: center,
        scale: 1.0,
        imp: match model {
            VoxelModel::Empty => {
                println!("Empty model: {} {}", generator, seed);
                ObjectImp::Empty
            }
            VoxelModel::Group(group) => ObjectImp::Group(group),
            VoxelModel::VoxelSet(voxel_set) => ObjectImp::VoxelSet(voxel_set),
            VoxelModel::VoxelScene(model) => {
                let mut group = Group::new();
                for layer in &model.layers {
                    for object in &layer.models {
                        println!("{} {:#?}", &object.model_id, &object.params);
                        let obj = generate(
                            object.model_id.clone().as_str(),
                            object.seed,
                            object.position,
                            object.params.clone(),
                            scene,
                        );
                        group.objects.push(obj);
                    }
                }
                ObjectImp::Group(Box::new(group))
            }
        },
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

    let z = z / 1.5;

    let mut transform = camera_query.single_mut();
    *transform = Transform::from_xyz(x, y, z).looking_at(state.look_at, Vec3::Z);
}

fn update_models(
    mut query_billboards: Query<(&mut Transform, &VoxelBillboard), With<VoxelBillboard>>,
) {
    for (mut transform, billboard) in query_billboards.iter_mut() {
        transform.rotate(Quat::from_rotation_z(billboard.z_rotation));
    }
}
