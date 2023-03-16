pub mod components;
mod systems;
mod utils;

pub use utils::*;

use bevy::app::App;
pub use components::*;
use bevy::prelude::*;
use crate::ServerSet::ServerUpdate;
use crate::utils::networking::messages::PlayerId;
use crate::simulation::ObjectId;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<OnPlayerDeathEvent>()
            .add_event::<OnKillEvent>()
            .add_event::<OnHealthChangedEvent>()
            .add_systems(
                (
                    systems::player_move,
                    systems::player_turret_rotate
                ).in_set(ServerUpdate)
            );
    }
}

pub struct OnPlayerDeathEvent {
    pub player_id: PlayerId,
}

pub struct OnKillEvent {
    pub attacker_id: PlayerId,
    pub victim_id: PlayerId,
}

pub struct OnHealthChangedEvent {
    pub object_id: ObjectId,
    pub health: f32,
    pub max_health: f32,
}
