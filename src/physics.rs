use bevy::app::{App, Plugin};
use bevy::math::Vec2;
use bevy::utils::default;
use bevy_rapier2d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin, Velocity};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            // physics_pipeline_active: false,
            ..default()
        });

        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.));

        #[cfg(debug_assertions)]
        app
            // .add_plugin(RapierDebugRenderPlugin::default())
            .register_type::<Velocity>();
    }
}