use bevy::app::{App, Plugin};
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::utils::default;
use bevy_rapier2d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};
use crate::AppState;
use crate::utils::networking::{is_client_connected, is_server_listening};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        });

        app
            .add_systems(
                (
                    disable_physics.run_if(is_client_connected.and_then(not(is_server_listening))),
                    enable_physics.run_if(is_server_listening),
                ).in_schedule(OnEnter(AppState::InGame))
            );

        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(150.));
    }
}

fn disable_physics(mut phys_config: ResMut<RapierConfiguration>) {
    phys_config.physics_pipeline_active = false;
}

fn enable_physics(mut phys_config: ResMut<RapierConfiguration>) {
    phys_config.physics_pipeline_active = true;
}