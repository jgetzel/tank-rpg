pub mod bundles;
pub mod components;
mod systems;

use bevy::app::App;
pub use components::*;
use bevy::math::Vec2;
use bevy::prelude::{BuildChildren, Commands, Entity, Plugin, SystemLabel};
use crate::networking::messages::PlayerId;
use crate::object::ObjectId;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerSpawnEvent>()
            .add_system(systems::player_move)
            .add_system(systems::player_turret_rotate)
            .add_system(systems::init_player);
    }
}

pub fn spawn_new_player(commands: &mut Commands, id: u64, pos: Option<Vec2>) -> Entity {
    commands.spawn(bundles::get_player_bundle(id, pos))
        .with_children(|p| {
            p.spawn(bundles::get_turret_bundle());
        }).id()
}

pub struct PlayerSpawnEvent {
    pub player_id: PlayerId,
    pub object_id: ObjectId,
}

#[derive(SystemLabel)]
enum PlayerJoinSysLabel {
    SpawnPlayer,
    ConfigPlayer,
}
