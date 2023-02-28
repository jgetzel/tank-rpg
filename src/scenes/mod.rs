mod loading;
mod main_menu;

use bevy::app::App;
use bevy::prelude::{Plugin};
use crate::scenes::loading::LoadingPlugin;
use crate::scenes::main_menu::MainMenuPlugin;

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
            .add_plugin(LoadingPlugin)
            .add_plugin(MainMenuPlugin);
    }
}
