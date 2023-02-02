use bevy::prelude::{Commands, default, Res, ResMut, Transform};
use bevy::sprite::SpriteBundle;
use bevy_rapier2d::plugin::RapierConfiguration;
use bevy::math::Vec2;
use crate::assets::GameAssets;

pub const BACKGROUND_LAYER: f32 = -10.;
pub const PLAYER_LAYER: f32 = 0.;
pub const BULLET_LAYER: f32 = 1.;
pub const TURRET_LAYER: f32 = 2.;
pub const CAMERA_LAYER: f32 = 100.;

pub fn remove_gravity(mut config: ResMut<RapierConfiguration>) {
    config.gravity = Vec2::new(0., 0.);
}

pub fn init_background(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn(SpriteBundle {
        texture: game_assets.background.clone(),
        transform: Transform::from_xyz(0., 0., BACKGROUND_LAYER),
        ..default()
    });
}
