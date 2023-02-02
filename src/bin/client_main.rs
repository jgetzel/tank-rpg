use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::utils::default;
use bevy::window::WindowPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_renet::{RenetClientPlugin, run_if_client_connected};
use tank_rpg::assets::{AppState, AssetsLoading, check_assets_loaded, GameAssets, load_assets};
use tank_rpg::camera::{camera_move, init_camera, you_tag_adder};
use tank_rpg::environment::init_background;
use tank_rpg::input_helper::{keyboard_events, mouse_position, PlayerInput};
use tank_rpg::networking::{client, Lobby};
use tank_rpg::networking::client::{PlayerJoinEvent, PlayerLeaveEvent, PlayerUpdateEvent};
use tank_rpg::player::{init_player, player_pos_updater, player_sprite_spawner};
use crate::ClientEventSysLabel::{ClientReceive, ClientSend};
use crate::PlayerJoinSysLabel::{ConfigPlayer, SpawnPlayer};

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(get_window_plugin())
            .set(get_log_plugin()))
        .add_plugin(RenetClientPlugin::default())
        .add_state(AppState::Loading)
        .insert_resource(client::new_client())
        .insert_resource(Lobby::default())
        .insert_resource(GameAssets::default())
        .insert_resource(AssetsLoading::default())
        .insert_resource(PlayerInput::default())
        .add_system_set(
            SystemSet::on_enter(AppState::Loading)
                .with_system(load_assets)
        )
        .add_system_set(
            SystemSet::on_update(AppState::Loading)
                .with_system(check_assets_loaded)
        )
        .add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(init_background)
                .with_system(init_camera)
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(camera_move)
        )
        .add_system(keyboard_events)
        .add_system(mouse_position)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(run_if_client_connected)
                .with_system(client::client_recv.label(ClientReceive))
                .with_system(client::client_send_input.
                    label(ClientSend).after(ClientReceive))
        );

    app.add_event::<PlayerJoinEvent>()
        .add_event::<PlayerLeaveEvent>()
        .add_event::<PlayerUpdateEvent>()
        .add_system(init_player.label(SpawnPlayer).after(ClientReceive))
        .add_system_set(
            SystemSet::new()
                .label(ConfigPlayer)
                .after(SpawnPlayer)
                .with_system(you_tag_adder)
                .with_system(player_sprite_spawner)
                .with_system(player_pos_updater)
        );

    app.add_plugin(WorldInspectorPlugin);

    app.run();
}

#[derive(SystemLabel)]
enum PlayerJoinSysLabel {
    SpawnPlayer,
    ConfigPlayer,
}

#[derive(SystemLabel)]
enum ClientEventSysLabel {
    ClientReceive,
    ClientSend,
}

fn get_log_plugin() -> LogPlugin {
    // this code is compiled only if debug assertions are enabled (debug mode)
    #[cfg(debug_assertions)]
    return LogPlugin {
        level: bevy::log::Level::DEBUG,
        filter: "debug,wgpu_core=warn,wgpu_hal=warn,mygame=debug".into(),
    };

    // this code is compiled only if debug assertions are disabled (release mode)
    #[cfg(not(debug_assertions))]
    return LogPlugin {
        level: bevy::log::Level::INFO,
        filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
    };
}

fn get_window_plugin() -> WindowPlugin {
    WindowPlugin {
        window: WindowDescriptor {
            title: "Client Window".into(),
            ..default()
        },
        ..default()
    }
}
