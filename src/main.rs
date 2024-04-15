use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

mod utils;
use crate::utils::{
    asset_loader::AssetsPlugin,
    kenney_asset::KenneyAssetPlugin
};   

mod gameui;
use crate::gameui::menu::MainMenuPlugin;


#[derive(
    Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States,
)]
pub enum GameState {
    #[default]
    AssetLoading,
    Menu,
    ChooseShip,
    Playing,
}

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

            AssetsPlugin,
            MainMenuPlugin,
        ))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}