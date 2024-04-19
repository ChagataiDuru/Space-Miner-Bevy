use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

use crate::{
    entities::{ 
        meteor::{
            Meteor, MeteorDestroyed, MeteorType
        },
        spaceship::ShipDestroyed,
    },
    controller::Laser,
    Player
};   

pub fn laser_meteor_collision(
    mut commands: Commands,
    mut meteor_destroyed: EventWriter<MeteorDestroyed>,
    lasers: Query<Entity, With<Laser>>,
    meteors: Query<
        (
            Entity,
            &CollidingEntities,
            &MeteorType,
            &Transform,
        ),
        With<Meteor>,
    >,
) {
    for (
        entity_meteor,
        colliding_entities,
        meteor_type,
        transform,
    ) in &meteors
    {
        if !colliding_entities.is_empty() {
            for entity_laser in &lasers {
                if colliding_entities
                    .contains(&entity_laser)
                {
                    commands
                        .entity(entity_laser)
                        .despawn_recursive();
                    commands
                        .entity(entity_meteor)
                        .despawn_recursive();

                    meteor_destroyed.send(
                        MeteorDestroyed {
                            destroyed_at: *transform,
                            destroyed_type: *meteor_type,
                        },
                    );
                }
            }
        }
    }
}

pub fn ship_meteor_collision(
    mut commands: Commands,
    mut ship_destroyed: EventWriter<ShipDestroyed>,
    meteors: Query<Entity, With<Meteor>>,
    player_ship: Query<
        (
            Entity,
            &CollidingEntities,
            &Transform,
        ),
        With<Player>,
    >,
) {
    for (
        entity_player,
        colliding_entities,
        transform,
    ) in &player_ship
    {
        if !colliding_entities.is_empty() {
            for entity_meteor in &meteors {
                if colliding_entities
                    .contains(&entity_meteor)
                {
                    commands
                        .entity(entity_player)
                        .despawn_recursive();

                    ship_destroyed.send(ShipDestroyed {
                        destroyed_at: *transform,
                    });
                }
            }
        }
    }
}