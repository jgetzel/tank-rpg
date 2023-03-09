use std::default::Default;
use std::env;
use bevy::a11y::AccessibilityPlugin;
use bevy::app::{App, Plugin, PluginGroupBuilder, ScheduleRunnerPlugin};
use bevy::audio::AudioPlugin;
use bevy::core_pipeline::CorePipelinePlugin;
use bevy::diagnostic::DiagnosticsPlugin;
use bevy::input::InputPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::scene::ScenePlugin;
use bevy::sprite::SpritePlugin;
use bevy::text::TextPlugin;
use bevy::time::TimePlugin;
use bevy::winit::WinitPlugin;
use bevy_egui::EguiPlugin;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use scenes::AppState;
use crate::asset_loader::AssetLoaderPlugin;
use crate::bullet::BulletPlugin;
use crate::camera::GameCameraPlugin;
use crate::networking::client::ClientPlugin;
use crate::networking::{Lobby, NetworkPlugin};
use crate::networking::server::ServerPlugin;
use crate::object::SyncedObjects;
use crate::physics::PhysicsPlugin;
use crate::player::{PlayerInput, PlayerPlugin};
use crate::scenes::TankScenePlugin;
use crate::sprite_updater::SpriteUpdatePlugin;

mod asset_loader;
mod object;
mod camera;
mod player;
mod bullet;
mod networking;
mod physics;
mod sprite_updater;
mod prefabs;
mod scenes;
mod utils;

pub struct ClientExecutablePlugin;

impl Plugin for ClientExecutablePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(DefaultExecutablePlugin)
            .add_plugin(ClientPlugin);
    }
}

pub struct ServerExecutablePlugin;

impl Plugin for ServerExecutablePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(DefaultExecutablePlugin)
            .add_plugin(ServerPlugin);
    }
}

struct DefaultExecutablePlugin;

impl Plugin for DefaultExecutablePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BevyDefaultPlugins);

        app.insert_resource(Lobby::default())
            .insert_resource(SyncedObjects::default());

        app.add_plugin(TankScenePlugin)
            .add_plugin(NetworkPlugin)
            .add_plugin(AssetLoaderPlugin)
            .add_plugin(GameCameraPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(BulletPlugin)
            .add_plugin(PhysicsPlugin)
            .add_plugin(SpriteUpdatePlugin);

        #[cfg(debug_assertions)]
        app
            .register_type::<PlayerInput>();
    }
}

struct BevyDefaultPlugins;

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

        let headless = is_headless();
        if headless {
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
