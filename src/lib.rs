#![feature(let_chains)]

use bevy::prelude::*;
use std::default::Default;
use std::env;
use bevy::a11y::AccessibilityPlugin;
use bevy::app::{PluginGroupBuilder, ScheduleRunnerPlugin};
use bevy::audio::AudioPlugin;
use bevy::core_pipeline::CorePipelinePlugin;
use bevy::diagnostic::DiagnosticsPlugin;
use bevy::input::InputPlugin;
use bevy::log::LogPlugin;
use bevy::render::RenderPlugin;
use bevy::scene::ScenePlugin;
use bevy::sprite::SpritePlugin;
use bevy::text::TextPlugin;
use bevy::time::TimePlugin;
use bevy::winit::WinitPlugin;
use bevy_egui::EguiPlugin;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use crate::asset_loader::AssetLoaderPlugin;
use crate::client_networking::ClientNetworkingPlugin;
use simulation::server_sim::player::PlayerInput;
use crate::client_ui::ClientUIPlugin;
use crate::ClientSet::{ClientReceive, ClientSend, ClientUpdate};
use crate::server_networking::ServerNetworkingPlugin;
use crate::simulation::SimulationPlugin;
use crate::display::DisplayPlugin;
use crate::server_ui::ServerUIPlugin;
use crate::ServerSet::{ServerReceive, ServerSend, ServerUpdate};
use crate::utils::networking::{is_client_connected, is_server_listening};

mod asset_loader;
mod utils;
mod client_networking;
mod server_networking;
mod simulation;
mod display;
mod client_ui;
mod server_ui;

pub struct ClientExecutablePlugin;

impl Plugin for ClientExecutablePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(DefaultExecutablePlugin)
            .add_plugin(ClientUIPlugin)
            .add_plugin(ClientNetworkingPlugin)
            .add_plugin(ServerNetworkingPlugin);
    }
}

pub struct ServerExecutablePlugin;

impl Plugin for ServerExecutablePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(DefaultExecutablePlugin)
            .add_plugin(ServerUIPlugin)
            .add_plugin(ServerNetworkingPlugin);
    }
}

struct DefaultExecutablePlugin;

impl Plugin for DefaultExecutablePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BevyDefaultPlugins { headless: is_headless()});

        app.add_state::<AppState>();

        app
            .add_plugin(SimulationPlugin)
            .add_plugin(DisplayPlugin)
            .add_plugin(AssetLoaderPlugin);

        //TODO this is very bad, please move to another plugin. Make one up, do what you have to do.
        app
            .configure_set(ServerUpdate.in_base_set(CoreSet::Update))
            .configure_set(ServerReceive.before(ServerUpdate)
                .run_if(is_server_listening))
            .configure_set(ServerUpdate.before(ServerSend)
                .run_if(is_server_listening))
            .configure_set(ServerSend
                .run_if(is_server_listening))
            .configure_set(ClientReceive.before(ClientUpdate).run_if(is_client_connected))
            .configure_set(ClientUpdate.before(ClientSend)
                .run_if(is_client_connected.and_then(not(is_server_listening))))
            .configure_set(ClientSend.run_if(is_client_connected));

        #[cfg(debug_assertions)]
        app
            .register_type::<PlayerInput>();
    }
}

#[derive(States, Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu,
    InGame,
    Paused,
}

#[allow(clippy::enum_variant_names)]
#[derive(SystemSet, Clone, Hash, Eq, PartialEq, Debug)]
pub enum ClientSet {
    ClientReceive,
    ClientUpdate,
    ClientSend,
}

#[allow(clippy::enum_variant_names)]
#[derive(SystemSet, Clone, Hash, Eq, PartialEq, Debug)]
pub enum ServerSet {
    ServerReceive,
    ServerUpdate,
    ServerSend,
}

struct BevyDefaultPlugins {
    pub headless: bool
}

impl PluginGroup for BevyDefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>()
            .add(TaskPoolPlugin::default())
            .add(TypeRegistrationPlugin::default())
            .add(FrameCountPlugin::default())
            .add(TimePlugin::default())
            .add(get_log_plugin())
            .add(TransformPlugin::default())
            .add(HierarchyPlugin::default())
            .add(DiagnosticsPlugin::default())
            .add(EmbeddedAssetPlugin::default())
            .add(AssetPlugin::default())
            .add(ScenePlugin::default());

        if self.headless {
            group = group.add(ScheduleRunnerPlugin::default());
        } else {
            group = group.add(InputPlugin::default())
                .add(WindowPlugin::default())
                .add(AccessibilityPlugin)
                .add(WinitPlugin::default())
                .add(RenderPlugin::default())
                .add(ImagePlugin::default())
                .add(CorePipelinePlugin::default())
                .add(SpritePlugin::default())
                .add(TextPlugin::default())
                .add(AudioPlugin::default())
                .add(GilrsPlugin::default())
                .add(AnimationPlugin::default())
                .add(EguiPlugin);
        }

        group
    }
}

fn is_headless() -> bool {
    env::args().any(|arg| arg == "headless")
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
    LogPlugin {
        level: bevy::log::Level::INFO,
        filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
    }
}

