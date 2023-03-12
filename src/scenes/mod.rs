pub mod in_game;

use bevy::app::{App, Plugin};
use bevy::prelude::{Commands, States, World};
use bevy::window::Window;
use crate::utils::despawn::CustomDespawnExt;

#[derive(States, Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu,
    InGame,
}

pub struct TankScenePlugin;

impl Plugin for TankScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_plugin(in_game::InGamePlugin);
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