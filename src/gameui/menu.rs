use bevy::prelude::*;

use crate::{
    utils::asset_loader::{
        FontAssets,
        ImageAssets,
    },
    GameState,
}; 

use crate::gameui::button::SpawnButton;

#[derive(Component)]
pub struct MainMenu;

#[derive(Resource, Component, Debug, PartialEq)]
pub enum MenuPage {
    Main,
    Settings,
}

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MenuPage::Main)
        .add_systems(
            OnExit(GameState::AssetLoading),
            main_menu
        )
        .add_systems(
            OnEnter(GameState::Menu),
            show_menu
        )
        .add_systems(
            OnExit(GameState::Menu), 
            hide_menu
        );
    }
}

fn show_menu(
    mut menu: Query<&mut Visibility, With<MainMenu>>,
) {
    let mut menu = menu.single_mut();
    *menu = Visibility::Visible;
}

fn hide_menu(
    mut menu: Query<&mut Visibility, With<MainMenu>>,
) {
    let mut menu = menu.single_mut();
    *menu = Visibility::Hidden;
}

pub fn main_menu(
    mut commands: Commands,
    images: Res<ImageAssets>,
    fonts: Res<FontAssets>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.),
                    width: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            let panel_slicer = TextureSlicer {
                border: BorderRect::square(20.0),
                center_scale_mode: SliceScaleMode::Stretch,
                sides_scale_mode: SliceScaleMode::Stretch,
                max_corner_scale: 1.0,
            };
     
            parent
                .spawn((
                    ImageBundle {
                        image: images.panel_glass.clone().into(),
                        style: Style {
                            width: Val::Px(360.0),
                            height: Val::Px(500.0),
                            flex_direction:
                                FlexDirection::Column,
                            justify_content:
                                JustifyContent::SpaceEvenly,
                            position_type:
                                PositionType::Absolute,
                            align_self: AlignSelf::Center,
                            border: UiRect::all(Val::Px(
                                10.0,
                            )),
                            ..default()
                        },
                        ..default()
                    },
                    ImageScaleMode::Sliced(panel_slicer.clone()),
                    MenuPage::Main,
                ))
                .with_children(|parent| {
                    let entity = parent.parent_entity();
                    parent.add_command(SpawnButton{
                        parent: entity,
                        text: "New Game"
                    });
                    parent.add_command(SpawnButton{
                        parent: entity,
                        text: "Settings"
                    });
                    parent.add_command(SpawnButton{
                        parent: entity,
                        text: "Exit"
                    });
                });
            parent
                .spawn((
                    ImageBundle {
                        image: images.panel_glass.clone().into(),
                        visibility: Visibility::Hidden,
                        style: Style {
                            width: Val::Px(360.0),
                            height: Val::Px(500.0),
                            flex_direction:
                                FlexDirection::Column,
                            justify_content:
                                JustifyContent::SpaceBetween,
                            border: UiRect::all(Val::Px(
                                10.0,
                            )),
                            ..default()
                        },
                        ..default()
                    },
                    ImageScaleMode::Sliced(panel_slicer.clone()),
                    MenuPage::Settings,
                ))
                .with_children(|parent| {
                    let entity = parent.parent_entity();
                    parent.add_command(SpawnButton{
                        parent: entity,
                        text: "Back"
                    });
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Auto,
                                height: Val::Px(25.0),
                                ..default()
                            },
                            ..default()
                        });
                });
        });
}