use bevy::prelude::{ Commands, default, Res, Transform};
use bevy::sprite::{ SpriteBundle };
use crate::assets::GameAssets;

pub static BACKGROUND_LAYER: f32 = -10.;
pub static PLAYER_LAYER: f32 = 0.;
pub static BULLET_LAYER: f32 = 1.;
pub static TURRET_LAYER: f32 = 2.;
pub static CAMERA_LAYER: f32 = 100.;

pub fn init_background(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn(SpriteBundle {
        texture: game_assets.background.clone(),
        transform: Transform::from_xyz(0., 0., BACKGROUND_LAYER),
        ..default()
    });
}