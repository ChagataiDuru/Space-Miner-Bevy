use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

use crate::{
    movement::MovementWrapper,
    Player
}; 

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app
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
    pub wrapping_movement: MovementWrapper,
}

#[derive(Component, Clone)]
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
