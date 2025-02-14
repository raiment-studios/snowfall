pub use crate::internal::*;

pub fn run() {
    println!("Hello, Bevy!");

    let mut app = App::new();
    app //
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            level: bevy::log::Level::WARN,
            ..default()
        }))
        .run();
}
