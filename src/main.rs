use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0., 0., 0.1,
        )))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Asteroids!".into(),
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}