use bevy::app::App;
use bevy::prelude::*;
use std::collections::HashMap;
use crate::AppState;
use crate::ServerSet::ServerUpdate;
use crate::simulation::events::OnRespawnTimerFinish;
use crate::simulation::server_sim::bullet::BulletSystemStage::CollisionHandle;
use crate::utils::networking::messages::PlayerId;

mod systems;

pub struct RespawnPlugin;

impl Plugin for RespawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(RespawnTimer::default())
            .add_event::<OnRespawnTimerFinish>()
            .add_systems(
                (
                    systems::start_respawn_timer_on_death.after(CollisionHandle),
                    systems::run_respawn_timer,
                    systems::dispatch_respawn_on_countdown,
                ).chain().in_set(ServerUpdate)
            )
            .add_system(systems::clear_respawns_on_match_end
                .in_schedule(OnExit(AppState::InGame)));

    }
}

#[derive(Resource, Default)]
pub struct RespawnTimer {
    pub map: HashMap<PlayerId, f32>,
}
