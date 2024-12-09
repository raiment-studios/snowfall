mod internal {
    pub use bevy::prelude::*;
    pub use std::collections::HashMap;

    pub use snowfall_voxel::prelude::*;
}

use crate::internal::*;

fn main() {
    let mut app = App::new();
    app //
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            level: bevy::log::Level::WARN,
            ..default()
        }))
        .add_systems(
            Startup,
            (
                startup, //
                startup_model,
            ),
        )
        .run();
}

fn generate_model() -> VoxelSet {
    let mut model = VoxelSet::new();
    model.register_block(Block::color("grass", 50, 200, 50));
    model.register_block(Block::color("sand", 180, 200, 20));

    for z in 0..5 {
        for y in -5..=5 {
            for x in -5..=5 {
                let x = x as i32;
                let y = y as i32;
                let z = z as i32;
                let p = ((x.abs() % 2) + (y.abs() % 2)) % 2;
                if p == (z.abs() % 2) {
                    continue;
                }
                let name = if (y.abs() % 2) == 0 { "grass" } else { "sand" };
                model.set_voxel((x, y, z), name);
            }
        }
    }
    model
}

fn startup_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let model = generate_model();

    let mut cache = HashMap::<String, Handle<StandardMaterial>>::new();
    for (VSVec3 { x, y, z }, block) in model.voxel_iter() {
        let material = match cache.get(&block.id) {
            Some(m) => m.clone(),
            None => {
                let rgb = match block.shader {
                    BlockShader::RGB(ref rgb) => rgb.clone(),
                    _ => BlockRGB { r: 0, g: 0, b: 0 },
                };
                let material = materials.add(Color::srgb(
                    rgb.r as f32 / 255.0,
                    rgb.g as f32 / 255.0,
                    rgb.b as f32 / 255.0,
                ));
                cache.insert(block.id.clone(), material.clone());
                material
            }
        };

        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(material),
            Transform::from_xyz(x as f32 + 0.5, y as f32 + 0.5, z as f32 + 0.5),
        ));
    }
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    use std::f32::consts::{FRAC_PI_2, PI};

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
