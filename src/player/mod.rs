pub mod components;
mod systems;
mod utils;

pub use utils::*;

use bevy::app::App;
pub use components::*;
use bevy::math::Vec2;
use bevy::prelude::{BuildChildren, Commands, Entity, IntoSystemConfig, Plugin};
use crate::bullet::BulletSystemStage::CollisionHandle;
use crate::networking::messages::PlayerId;
use crate::player::systems::death_reader;
use crate::prefabs;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DeathEvent>()
            .add_system(systems::player_move)
            .add_system(systems::player_turret_rotate)
            .add_system(death_reader.after(CollisionHandle));
    }
}

pub fn spawn_new_player(commands: &mut Commands, id: PlayerId, pos: Option<Vec2>) -> Entity {
    commands.spawn(prefabs::get_player_bundle(id, pos))
        .with_children(|p| {
            p.spawn(prefabs::get_turret_bundle());
        }).id()
}

pub struct DeathEvent {
    pub entity: Entity,
}
