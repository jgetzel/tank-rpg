use bevy::app::App;
use tank_rpg::ServerExecutablePlugin;

fn main() {
    App::new().add_plugin(ServerExecutablePlugin).run();
}