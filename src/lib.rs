use bevy::app::{App, Plugin};
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::{RapierDebugRenderPlugin, Velocity};
use crate::assets::{AppState, AssetLoaderPlugin};
use crate::camera::GameCameraPlugin;
use crate::environment::EnvironmentPlugin;
use crate::input_helper::{InputHelperPlugin, PlayerInput};
use crate::networking::client::ClientPlugin;
use crate::networking::{Lobby, NetworkPlugin};
use crate::networking::server::ServerPlugin;
use crate::object::SyncedObjects;
use crate::player::PlayerPlugin;

pub mod assets;
pub mod object;
pub mod camera;
pub mod environment;
pub mod input_helper;
pub mod player;
pub mod bullet;
pub mod networking;

pub struct ClientExecutablePlugin;

impl Plugin for ClientExecutablePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins
            .set(get_window_plugin("Client Window"))
            .set(get_log_plugin())
        )
            .add_plugin(DefaultExecutablePlugin)
            .add_plugin(ClientPlugin);
    }
}

pub struct ServerExecutablePlugin;

impl Plugin for ServerExecutablePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins
            .set(get_window_plugin("Server Window"))
            .set(get_log_plugin())
        )
            .add_plugin(DefaultExecutablePlugin)
            .add_plugin(ServerPlugin);
    }
}

struct DefaultExecutablePlugin;

impl Plugin for DefaultExecutablePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::Loading);
        app.insert_resource(Lobby::default())
            .insert_resource(SyncedObjects::default());

        app.add_plugin(AssetLoaderPlugin)
            .add_plugin(NetworkPlugin)
            .add_plugin(EnvironmentPlugin)
            .add_plugin(InputHelperPlugin)
            .add_plugin(GameCameraPlugin)
            .add_plugin(PlayerPlugin);

        #[cfg(debug_assertions)]
        app.add_plugin(WorldInspectorPlugin)
            .add_plugin(RapierDebugRenderPlugin::default())
            .register_type::<PlayerInput>()
            .register_type::<Velocity>();
    }
}

fn get_log_plugin() -> LogPlugin {
    // // this code is compiled only if debug assertions are enabled (debug mode)
    // #[cfg(debug_assertions)]
    // return LogPlugin {
    //     level: bevy::log::Level::DEBUG,
    //     filter: "debug,wgpu_core=warn,wgpu_hal=warn,mygame=debug".into(),
    // };
    //
    // // this code is compiled only if debug assertions are disabled (release mode)
    // #[cfg(not(debug_assertions))]
    return LogPlugin {
        level: bevy::log::Level::INFO,
        filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
    };
}

fn get_window_plugin(title: &str) -> WindowPlugin {
    WindowPlugin {
        window: WindowDescriptor {
            title: title.into(),
            ..default()
        },
        ..default()
    }
}