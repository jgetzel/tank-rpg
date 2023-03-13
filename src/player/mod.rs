pub mod components;
mod systems;
mod utils;

pub use utils::*;

use bevy::app::App;
pub use components::*;
use bevy::prelude::{Plugin};
use crate::networking::messages::PlayerId;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<OnPlayerDeathEvent>()
            .add_event::<OnKillEvent>()
            .add_system(systems::player_move)
            .add_system(systems::player_turret_rotate);
    }
}

pub struct OnPlayerDeathEvent {
    pub player_id: PlayerId,
}

pub struct OnKillEvent {
    pub attacker_id: PlayerId,
    pub victim_id: PlayerId,
}
