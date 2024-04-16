use std::f32::consts::TAU;
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use rand::prelude::*;

use crate::{
    utils::{
        asset_loader::ImageAssets,
        kenney_asset::KenneySpriteSheetAsset,
    },
    movement::{LinearMovement, Rotate2D, MovementWrapper},
    GameState,
};


pub struct MeteorPlugin;

impl Plugin for MeteorPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Bundle)]
pub struct MeteorBundle {
    meteor_type: MeteorType,
    meteor: Meteor,
    collider: Collider,
    sprite_bundle: SpriteBundle,
    texture_atlas: TextureAtlas,
    linear_movement: LinearMovement,
    spin: Rotate2D,
    wrapping: MovementWrapper,
}
#[derive(Component, Clone, Copy)]
pub enum MeteorType {
    Big,
    Medium,
    Small,
}
#[derive(Component)]
pub struct Meteor;

const METEOR_BASE_SPEED_BIG: f32 = 1.;
const METEOR_BASE_SPEED_MEDIUM: f32 = 1.2;
const METEOR_BASE_SPEED_SMALL: f32 = 1.4;

impl MeteorBundle {
    pub fn big(
        transform: Transform,
        space_sheet: &KenneySpriteSheetAsset,
    ) -> MeteorBundle {
        let mut rng = rand::thread_rng();
        let x = rng.gen::<f32>() * METEOR_BASE_SPEED_BIG;
        let y = rng.gen::<f32>() * METEOR_BASE_SPEED_BIG;
        let rotation = rng.gen::<f32>() * TAU;

        MeteorBundle {
            meteor_type: MeteorType::Big,
            meteor: Meteor,
            collider: Collider::circle(42.),
            sprite_bundle: SpriteBundle {
                transform,
                texture: space_sheet.sheet.clone(),
                ..default()
            },
            texture_atlas: TextureAtlas {
                index: 163,
                layout: space_sheet
                    .texture_atlas_layout
                    .clone(),
            },
            linear_movement: LinearMovement {
                movement_factor: Vec2::new(
                    x as f32, y as f32,
                ),
                movement_direction: Quat::from_rotation_z(
                    rotation,
                ),
            },
            spin: Rotate2D(1.3),
            wrapping: MovementWrapper,
        }
    }
    pub fn medium(
        transform: Transform,
        space_sheet: &KenneySpriteSheetAsset,
    ) -> MeteorBundle {
        let mut rng = rand::thread_rng();
        let x = rng.gen::<f32>() * METEOR_BASE_SPEED_MEDIUM;
        let y = rng.gen::<f32>() * METEOR_BASE_SPEED_MEDIUM;
        let rotation = rng.gen::<f32>() * TAU;

        MeteorBundle {
            meteor_type: MeteorType::Medium,
            meteor: Meteor,
            collider: Collider::circle(21.),
            sprite_bundle: SpriteBundle {
                transform,
                texture: space_sheet.sheet.clone(),
                ..default()
            },
            texture_atlas: TextureAtlas {
                index: 167,
                layout: space_sheet
                    .texture_atlas_layout
                    .clone(),
            },
            linear_movement: LinearMovement {
                movement_factor: Vec2::new(
                    x as f32, y as f32,
                ),
                movement_direction: Quat::from_rotation_z(
                    rotation,
                ),
            },
            spin: Rotate2D(1.6),
            wrapping: MovementWrapper,
        }
    }
    pub fn small(
        transform: Transform,
        space_sheet: &KenneySpriteSheetAsset,
    ) -> MeteorBundle {
        let mut rng = rand::thread_rng();
        let x = rng.gen::<f32>() * METEOR_BASE_SPEED_SMALL;
        let y = rng.gen::<f32>() * METEOR_BASE_SPEED_SMALL;
        let rotation = rng.gen::<f32>() * TAU;

        MeteorBundle {
            meteor_type: MeteorType::Small,
            meteor: Meteor,
            collider: Collider::circle(14.),
            sprite_bundle: SpriteBundle {
                transform,
                texture: space_sheet.sheet.clone(),
                ..default()
            },
            texture_atlas: TextureAtlas {
                index: 169,
                layout: space_sheet
                    .texture_atlas_layout
                    .clone(),
            },
            linear_movement: LinearMovement {
                movement_factor: Vec2::new(
                    x as f32, y as f32,
                ),
                movement_direction: Quat::from_rotation_z(
                    rotation,
                ),
            },
            spin: Rotate2D(2.),
            wrapping: MovementWrapper,
        }
    }
}