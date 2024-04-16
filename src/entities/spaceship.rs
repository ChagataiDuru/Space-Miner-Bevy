use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

mod movement;
use crate::{
    utils::{
    asset_loader::ImageAssets, 
    kenney_asset::KenneySpriteSheetAsset
    },
    GameState,Player
}; 

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            player_ship_destroyed_event_handler
                .run_if(resource_equals(
                    Pausable::NotPaused,
                ))
                .run_if(in_state(GameState::Playing)),
        )
        .add_event::<ShipDestroyed>();
    }
}