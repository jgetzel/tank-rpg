use bevy::app::App;
use bevy::prelude::Plugin;
use crate::client_ui::client_debug::ClientDebugUIPlugin;
use crate::client_ui::health::HealthUiPlugin;
use crate::client_ui::leaderboard::LeaderboardUIPlugin;
use crate::client_ui::main_menu::MainMenuPlugin;

pub mod client_debug;
pub mod health;
pub mod leaderboard;
pub mod main_menu;

pub struct ClientUIPlugin;

impl Plugin for ClientUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(MainMenuPlugin)
            .add_plugin(HealthUiPlugin)
            .add_plugin(LeaderboardUIPlugin);

        #[cfg(debug_assertions)]
        app.add_plugin(ClientDebugUIPlugin);
    }
}
