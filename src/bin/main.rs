use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierPhysicsPlugin;
use bevy_rapier2d::prelude::NoUserData;
use tank_rpg::assets::{AppState, AssetsLoading, check_assets_loaded, GameAssets, load_assets};
use tank_rpg::bullet::fire_bullet;
use tank_rpg::camera::{camera_move, init_camera};
use tank_rpg::environment;
use tank_rpg::environment::init_background;
use tank_rpg::input_helper::{keyboard_events, mouse_position, PlayerInput};
use tank_rpg::player::{player_move, player_turret_rotate};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        // .add_plugin(WorldInspectorPlugin) //TODO REMOVE ON RELEASE
        // .add_plugin(RapierDebugRenderPlugin::default())//TODO REMOVE ON RELEASE
        .add_state(AppState::Loading)
        .insert_resource(GameAssets::default())
        .insert_resource(AssetsLoading::default())
        .insert_resource(PlayerInput::default())
        .add_system(keyboard_events)
        .add_system(mouse_position)
        .add_system_set(
            SystemSet::on_enter(AppState::Loading)
                .with_system(load_assets)
                .with_system(environment::remove_gravity)
        )
        .add_system_set(
            SystemSet::on_update(AppState::Loading)
                .with_system(check_assets_loaded)
        )
        .add_system_set(
            SystemSet::on_enter(AppState::InGame)
                // .with_system(init_player(0,Vec2::default()))
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