use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::render::RapierDebugRenderPlugin;
use bevy::prelude::*;
use bevy_rapier2d::plugin::{RapierConfiguration, RapierPhysicsPlugin};
use bevy_rapier2d::prelude::NoUserData;
use crate::game_plugins::assets::{AssetsLoading, check_assets_loaded, GameAssets, load_assets};
use crate::game_plugins::bullet::fire_bullet;
use crate::game_plugins::camera::{camera_move, init_camera};
use crate::game_plugins::environment::{init_background};
use crate::game_plugins::input_helper;
use crate::game_plugins::input_helper::{keyboard_events, mouse_position};
use crate::game_plugins::player::{init_player, player_move, player_turret_rotate};

mod game_plugins;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Loading,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        // .add_plugin(WorldInspectorPlugin) //TODO REMOVE ON RELEASE
        // .add_plugin(RapierDebugRenderPlugin::default())//TODO REMOVE ON RELEASE
        .add_state(AppState::Loading)
        .insert_resource(GameAssets::default())
        .insert_resource(AssetsLoading::default())
        .insert_resource(input_helper::Input::default())
        .add_system(keyboard_events)
        .add_system(mouse_position)
        .add_system_set(
            SystemSet::on_enter(AppState::Loading)
                .with_system(load_assets)
                .with_system(remove_gravity)
        )
        .add_system_set(
            SystemSet::on_update(AppState::Loading)
                .with_system(check_assets_loaded)
        )
        .add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(init_player(Vec2::default()))
                .with_system(init_background)
                .with_system(init_camera)
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(player_move)
                .with_system(player_turret_rotate)
                .with_system(camera_move)
                .with_system(fire_bullet)
        )
        .run();
}

fn remove_gravity(mut config: ResMut<RapierConfiguration>) {
    config.gravity = Vec2::new(0., 0.);
}