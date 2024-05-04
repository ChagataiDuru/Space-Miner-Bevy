use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use bevy_hanabi::prelude::*;

use crate::{
    movement::MovementWrapper,
    controller::MovementFactor,
    utils::{asset_loader::ImageAssets, kenney_asset::KenneySpriteSheetAsset, pause_system::Pausable},
    entities::lives::{Lives, RemoveLifeEvent},
    GameState,
    Player
}; 

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                PostUpdate,
                player_ship_destroyed_event_handler
                    .run_if(resource_equals(
                        Pausable::NotPaused,
                    ))
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                PostUpdate,
                spawn_ship_after_ship_destroyed
                    .run_if(in_state(GameState::Playing)),
            )
            .add_event::<ShipDestroyed>();
    }
}


#[derive(Component)]
pub struct EngineFire;

#[derive(Bundle)]
pub struct ShipBundle {
    pub sprite_bundle: SpriteBundle,
    pub texture_atlas: TextureAtlas,
    pub player: Player,
    pub ship_type: ShipLevels,
    pub collider: Collider,
    //pub wrapping_movement: MovementWrapper,
}

#[derive(Resource, Component, Clone)]
pub enum ShipLevels {
    Initial,
    Mid,
    Best,
}

pub struct BaseShipSpeed {
    pub movement_speed: f32, // linear speed in meters per second
    pub rotation_speed: f32, // angular speed in radians per second
}


impl ShipLevels {
    pub fn base_atlas_index(&self) -> usize {
        match &self {
            ShipLevels::Initial => 200,
            ShipLevels::Mid => 207,
            ShipLevels::Best => 214,
        }
    }
    pub fn life_atlas_index(&self) -> usize {
        match &self {
            ShipLevels::Initial => 188,
            ShipLevels::Mid => 192,
            ShipLevels::Best => 196,
        }
    }
    pub fn all_ships() -> Vec<ShipLevels> {
        vec![
            ShipLevels::Initial,
            ShipLevels::Mid,
            ShipLevels::Best,
        ]
    }
    pub fn collider(&self) -> Collider {
        Collider::capsule(40., 10.)
    }
    pub fn base_ship_speed(&self) -> BaseShipSpeed {
        match self {
            ShipLevels::Initial => BaseShipSpeed {
                movement_speed: 200.0, // meters per second
                rotation_speed: f32::to_radians(360.0), /* degrees per second */
            },
            ShipLevels::Mid => BaseShipSpeed {
                movement_speed: 350.0,
                rotation_speed: f32::to_radians(360.0),
            },
            ShipLevels::Best => BaseShipSpeed {
                movement_speed: 500.0,
                rotation_speed: f32::to_radians(360.0),
            },
        }
    }
}

#[derive(Event)]
pub struct ShipDestroyed {
    pub destroyed_at: Transform,
}

fn player_ship_destroyed_event_handler(
    mut commands: Commands,
    images: Res<ImageAssets>,
    mut events: EventReader<ShipDestroyed>,
    sheets: Res<Assets<KenneySpriteSheetAsset>>,
    mut effect: Query<(
        &mut EffectProperties,
        &mut EffectSpawner,
        &mut Transform,
    )>,
    mut ship_movement: ResMut<MovementFactor>,
    mut life_events: EventWriter<RemoveLifeEvent>,
) {
    let Some(space_sheet) = sheets.get(&images.space_sheet)
    else {
        warn!("player_ship_destroyed_event_handler requires meteor sprites to be loaded");
        return;
    };

    let Ok((
        mut properties,
        mut spawner,
        mut effect_transform,
    )) = effect.get_single_mut()
    else {
        warn!("effect not ready yet, returning");
        return;
    };

    for ShipDestroyed {
        destroyed_at,
    } in &mut events.read()
    {
        effect_transform.translation =
            destroyed_at.translation;

        let color = Color::lch(
            1.,
            1.,
            rand::random::<f32>() * 360.,
        )
        .as_linear_rgba_u32();
        properties.set("spawn_color", color.into());

        // Spawn the particles
        spawner.reset();

        ship_movement.0 = Vec2::ZERO;

        life_events.send(RemoveLifeEvent);
    }
}

fn spawn_ship_after_ship_destroyed(
    mut commands: Commands,
    images: Res<ImageAssets>,
    sheets: Res<Assets<KenneySpriteSheetAsset>>,
    lives: Res<Lives>,
) {
    if !lives.is_changed() || lives.0 == 0 || lives.0 == 3 {
        return;
    }
    let Some(space_sheet) = sheets.get(&images.space_sheet)
    else {
        warn!("player_ship_destroyed_event_handler requires meteor sprites to be loaded");
        return;
    };

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
        //wrapping_movement: MovementWrapper
    })
    .add_child(engine_fire)
    .add_child(right_truster)
    .add_child(left_truster);

}
