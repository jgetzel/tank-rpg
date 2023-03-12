mod systems;

use std::collections::HashMap;
use bevy::app::App;
use bevy::prelude::*;
use crate::bullet::BulletSystemStage::CollisionHandle;
use crate::networking::messages::PlayerId;
use crate::networking::server::ServerSet::ServerUpdate;
use crate::scenes::AppState;
use crate::scenes::in_game::systems::{dispatch_respawn_on_countdown, on_connect_spawn_player, respawn_reader, run_respawn_timer, start_respawn_timer_on_death};

pub struct InGamePlugin;


impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(RespawnTimer::default())
            .add_event::<RespawnEvent>()
            .add_system(systems::init_default.in_schedule(OnEnter(AppState::InGame)))
            .add_system(on_connect_spawn_player.in_set(ServerUpdate))
            .add_systems(
                (
                    start_respawn_timer_on_death.after(CollisionHandle),
                    run_respawn_timer,
                    dispatch_respawn_on_countdown,
                    respawn_reader
                ).chain().in_set(ServerUpdate)
            );
    }
}

#[derive(Resource, Default)]
pub struct RespawnTimer {
    pub map: HashMap<PlayerId, f32>,
}

pub struct RespawnEvent {
    player_id: PlayerId,
}
