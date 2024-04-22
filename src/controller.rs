use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use std::time::Duration;

use crate::{
    entities::spaceship::ShipLevels,
    entities::spaceship::EngineFire, 
    utils::{
        asset_loader::ImageAssets, 
        kenney_asset::KenneySpriteSheetAsset, 
        pause_system::Pausable
    }, GameState, Player
};   

#[derive(Component)]
pub struct Laser(Vec2);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct MovementFactor(pub Vec2);

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MovementFactor>().add_systems(
            Update,
            (
                player_movement_system
                    .run_if(in_state(GameState::Playing)),
                weapon_system
                    .run_if(in_state(GameState::Playing)),
                engine_fire
                    .run_if(in_state(GameState::Playing)),
                laser_movement,
            )
                .run_if(resource_equals(
                    Pausable::NotPaused,
                )),
        );
    }
}

fn engine_fire(
    mut query: Query<
        &mut Visibility,
        With<EngineFire>,
    >,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        for mut visibility in query.iter_mut() {
            *visibility = Visibility::Visible;
        }
    } else {
        for mut visibility in query.iter_mut() {
            *visibility = Visibility::Hidden;
        }
    }
}

const ROTATION_SPEED: f32 = 1.0;
const MOVEMENT_SPEED: f32 = 0.01;
const DECAY_FACTOR: f32 = 0.95;

fn handle_keyboard_input(keyboard_input: &Res<ButtonInput<KeyCode>>, key: KeyCode) -> f32 {
    if keyboard_input.pressed(key) {
        ROTATION_SPEED
    } else {
        0.0
    }
}

fn update_movement_factor(
    user_facing_direction: Vec2, 
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    movement_factor: &mut ResMut<MovementFactor> 
) {
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        movement_factor.0 = (movement_factor.0 + MOVEMENT_SPEED * user_facing_direction).clamp(Vec2::splat(-1.0), Vec2::splat(1.0)); // Move the ship
    } else {
        movement_factor.0 = (movement_factor.0 * DECAY_FACTOR).clamp(Vec2::splat(-1.0), Vec2::splat(1.0)); // Decay the movement factor
    }
}

fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &ShipLevels), With<Player>>,
    mut movement_factor: ResMut<MovementFactor>,
) {
    if let Ok((mut transform, ship)) = query.get_single_mut() {

        let rotation_factor = handle_keyboard_input(&keyboard_input, KeyCode::ArrowLeft) - handle_keyboard_input(&keyboard_input, KeyCode::ArrowRight);

        transform.rotate_z(rotation_factor * ship.base_ship_speed().rotation_speed * time.delta_seconds()); // Rotate the ship accross the z axis perpendicularly to the screen on 2d

        let user_facing_direction = (transform.rotation * Vec3::Y).xy(); // Get the direction the ship is facing

        update_movement_factor(user_facing_direction, &keyboard_input,&mut movement_factor,); // Update the movement factor

        let movement_distance = movement_factor.0 * ship.base_ship_speed().movement_speed * time.delta_seconds(); // Calculate the distance the ship should move

        let translation_delta = movement_distance;

        transform.translation.x += translation_delta.x;
        transform.translation.y += translation_delta.y;

    } else {
        // Handle the error case here

    }
}

fn laser_movement(
    mut lasers: Query<(&mut Transform, &Laser)>,
    time: Res<Time>,
) {
    for (
        mut transform,
        Laser(preexisting_movement_factor),
    ) in &mut lasers
    {
        let laser_facing_direction =
            transform.rotation * Vec3::Y;
        let translation_delta = *preexisting_movement_factor
            + laser_facing_direction.xy()
                * 1000.
                * time.delta_seconds();
        transform.translation.x += translation_delta.x;
        transform.translation.y += translation_delta.y;
    }
}

fn weapon_system(
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<&Transform, With<Player>>,
    movement_factor: ResMut<MovementFactor>,
    images: Res<ImageAssets>,
    sheets: Res<Assets<KenneySpriteSheetAsset>>,
    mut last_shot: Local<Option<Duration>>,
) {
    let space_sheet =
        sheets.get(&images.space_sheet).unwrap();

    let Ok(transform) = query.get_single() else {
        error!(
            "Only expected one Player component got {}",
            query.iter().count()
        );
        return;
    };

    if keyboard_input.pressed(KeyCode::Space) {
        let can_shoot = last_shot.is_none() || {
            if let Some(shot) = *last_shot {
                time.elapsed() - shot
                    > Duration::from_millis(200)
            } else {
                false
            }
        };

        if can_shoot {
            *last_shot = Some(time.elapsed());

            commands.spawn((
                SpriteBundle {
                    transform: *transform,
                    texture: space_sheet.sheet.clone(),
                    ..default()
                },
                TextureAtlas {
                    layout: space_sheet
                        .texture_atlas_layout
                        .clone(),
                    index: 105,
                },
                Laser(**movement_factor),
                Collider::triangle(
                    Vec2::new(0., -27.),
                    Vec2::new(4.5, 27.),
                    Vec2::new(-4.5, 27.),
                ),
            ));
        }
    }
}
