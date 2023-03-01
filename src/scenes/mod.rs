mod in_game;

use bevy::app::{App, Plugin};
use bevy::prelude::{Commands, World};
use crate::utils::CustomDespawn;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Loading,
    MainMenu,
    InGame,
}

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::Loading)
            .add_plugin(in_game::InGamePlugin);
    }
}

pub fn despawn_all_entities(
    mut commands: Commands,
    world: &World
) {
    world.iter_entities().for_each(|e| {
        commands.entity(e).custom_despawn();
    })
}