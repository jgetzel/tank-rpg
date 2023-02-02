use bevy::DefaultPlugins;
use bevy::prelude::{App, default, PluginGroup, WindowDescriptor, WindowPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::prelude::Velocity;
use bevy_renet::RenetServerPlugin;
use tank_rpg::environment;
use tank_rpg::input_helper::PlayerInput;
use tank_rpg::networking::{Lobby, server};
use tank_rpg::player::player_move;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "Server Window".into(),
            ..default()
        },
        ..default()
    }))
        .add_plugin(RenetServerPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .insert_resource(Lobby::default())
        .insert_resource(server::new_server())
        .add_startup_system(environment::remove_gravity)
        .add_system(server::server_recv)
        .add_system(server::broadcast_state)
        .add_system(server::force_disconnect_handler)
        .add_system(player_move);


    app.add_plugin(WorldInspectorPlugin)
        .register_type::<PlayerInput>()
        .register_type::<Velocity>();

    app.run();
}