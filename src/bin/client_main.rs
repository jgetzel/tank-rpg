use bevy::app::App;
use tank_rpg::ClientExecutablePlugin;

fn main() {
    App::new().add_plugin(ClientExecutablePlugin).run();
}
