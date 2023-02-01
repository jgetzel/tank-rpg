use bevy::DefaultPlugins;
use bevy::prelude::App;
use bevy_renet::RenetServerPlugin;
use tank_rpg::networking::server;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RenetServerPlugin::default())
        .insert_resource(server::new_server())
        .add_system(server::server_recv)
        // .add_system(server_sync_players)
        // .add_system(move_players_sys)
        .run();
}