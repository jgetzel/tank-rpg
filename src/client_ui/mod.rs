use bevy::app::App;
use bevy::prelude::{EventReader, IntoSystemConfig, NextState, OnUpdate, Plugin, ResMut};
use crate::AppState;
use crate::asset_loader::AssetsLoadedEvent;
use crate::client_ui::client_debug::ClientDebugUIPlugin;
use crate::client_ui::health::HealthUiPlugin;
use crate::client_ui::leaderboard::LeaderboardUIPlugin;
use crate::client_ui::main_menu::MainMenuPlugin;
use crate::client_ui::match_end_screen::MatchEndScreenUIPlugin;
use crate::client_ui::match_length::MatchLengthUIPlugin;

mod client_debug;
mod health;
mod leaderboard;
mod main_menu;
mod match_length;
mod match_end_screen;

pub struct ClientUIPlugin;

impl Plugin for ClientUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(MainMenuPlugin)
            .add_plugin(HealthUiPlugin)
            .add_plugin(LeaderboardUIPlugin)
            .add_plugin(MatchLengthUIPlugin)
            .add_plugin(MatchEndScreenUIPlugin);

        app.add_system(main_menu_on_load.in_set(OnUpdate(AppState::Loading)));

        #[cfg(debug_assertions)]
        app.add_plugin(ClientDebugUIPlugin);
    }
}


pub fn main_menu_on_load(
    mut evt: EventReader<AssetsLoadedEvent>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if evt.iter().next().is_some() {
        next_state.set(AppState::MainMenu);
    }
}
