use bevy::app::App;
use bevy::prelude::{Commands, Plugin, Window, World};
use crate::simulation::server_sim::bullet::BulletPlugin;
use crate::simulation::server_sim::init::InitPlugin;
use crate::simulation::server_sim::physics::PhysicsPlugin;
use crate::simulation::server_sim::player::PlayerPlugin;
use crate::simulation::server_sim::respawn::RespawnPlugin;
use crate::simulation::server_sim::spawn::SpawnPlugin;
use crate::utils::commands::despawn::CustomDespawnExt;

pub mod player;
pub mod bullet;
pub mod physics;
pub mod respawn;
pub mod spawn;
pub mod init;

pub struct ServerSimulationPlugin;

impl Plugin for ServerSimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(InitPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(BulletPlugin)
            .add_plugin(PhysicsPlugin)
            .add_plugin(RespawnPlugin)
            .add_plugin(SpawnPlugin);
    }
}

pub fn despawn_all_entities(
    mut commands: Commands,
    world: &World,
) {
    world.iter_entities().filter(|e| world.get::<Window>(e.id()).is_none())
        .for_each(|e| {
            commands.entity(e.id()).custom_despawn();
        })
}
