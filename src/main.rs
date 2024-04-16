use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

mod utils;
mod gameui;
mod movement;
mod entities;
use crate::{
    utils::asset_loader::AssetsPlugin,
    gameui::settings::SettingsPlugin,
    gameui::menu::MainMenuPlugin,
    movement::MovementPlugin,
    entities::spaceship::ShipPlugin,
};   

#[derive(
    Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States,
)]
pub enum GameState {
    #[default]
    AssetLoading,
    Menu,
    Playing,
}

#[derive(Component)]
pub struct Player;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0.078, 0.082, 0.188,
        )))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Space Miner".into(),
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),

            AssetsPlugin,
            SettingsPlugin,
            MainMenuPlugin,
            MovementPlugin,
            ShipPlugin,
        ))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}