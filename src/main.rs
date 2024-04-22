use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use bevy_hanabi::prelude::*;

mod utils;
mod gameui;
mod movement;
mod entities;
mod controller;
use crate::{
    utils::{
        asset_loader::AssetsPlugin,
        asset_loader::ImageAssets, 
        kenney_asset::KenneySpriteSheetAsset,
        pause_system::PausePlugin,
        pause_system::Pausable,
    },
    gameui::settings::SettingsPlugin,
    gameui::menu::MainMenuPlugin,
    movement::MovementPlugin,
    movement::MovementWrapper,
    entities::{
        spaceship::ShipPlugin,
        spaceship::ShipBundle,
        spaceship::ShipLevels,
        spaceship::EngineFire,
        meteor::MeteorPlugin,
        meteor::MeteorBundle,
        collisions::laser_meteor_collision,
        collisions::ship_meteor_collision,
    },
    controller::ControlsPlugin,
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
            HanabiPlugin,

            AssetsPlugin,
            SettingsPlugin,
            MainMenuPlugin,
            PausePlugin,
            MovementPlugin,
            ShipPlugin,
            MeteorPlugin,
            ControlsPlugin,
        ))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(GameState::Playing), test_game_start)
        .add_systems(
            Update,
            (
                laser_meteor_collision,
                ship_meteor_collision,
            )
                .run_if(in_state(GameState::Playing))
                .run_if(resource_equals(
                    Pausable::NotPaused,
                )),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()); 
}

fn test_game_start(
    mut commands: Commands,
    images: Res<ImageAssets>,
    sheets: Res<Assets<KenneySpriteSheetAsset>>
){
    let space_sheet = sheets.get(&images.space_sheet).unwrap();
    let engine_fire = commands
    .spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                0., -60., 1.,
            ),
            texture: space_sheet.sheet.clone(),
            sprite: Sprite {
                flip_y: true,
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        TextureAtlas {
            index: 74,
            layout: space_sheet
                .texture_atlas_layout
                .clone(),
        },
        EngineFire,
    ))
    .id();
    let right_truster = commands
    .spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                16., -40., 1.,
            ),
            texture: space_sheet.sheet.clone(),
            sprite: Sprite {
                flip_y: true,
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        TextureAtlas {
            index: 75,
            layout: space_sheet
                .texture_atlas_layout
                .clone(),
        },
        EngineFire,
    ))
    .id();
    let left_truster = commands
    .spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                -16., -40., 1.,
            ),
            texture: space_sheet.sheet.clone(),
            sprite: Sprite {
                flip_y: true,
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        TextureAtlas {
            index: 75,
            layout: space_sheet
                .texture_atlas_layout
                .clone(),
        },
        EngineFire,
    ))
    .id();
    commands.spawn(ShipBundle {
        sprite_bundle: SpriteBundle {
            texture: space_sheet.sheet.clone(),
            ..default()
        },
        texture_atlas: TextureAtlas {
            index: 200,
            layout: space_sheet
                    .texture_atlas_layout
                    .clone(),
        },
        player: Player,
        ship_type: ShipLevels::Initial,
        collider: Collider::circle(32.),
        wrapping_movement: MovementWrapper
    })
    .add_child(engine_fire)
    .add_child(right_truster)
    .add_child(left_truster);
    commands.spawn(MeteorBundle::big(
        Transform::from_xyz(50., 100., 1.),
        &space_sheet,
    ));
}