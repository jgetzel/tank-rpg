use bevy::app::{App, Plugin};
use bevy::prelude::{Camera2dBundle, Commands, default, Res, SystemSet, Transform};
use bevy::sprite::SpriteBundle;
use crate::assets::{AppState, GameAssets, SpriteEnum};
use crate::camera::MainCamera;

pub const BACKGROUND_LAYER: f32 = -10.;
pub const PLAYER_LAYER: f32 = 0.;
pub const BULLET_LAYER: f32 = 1.;
pub const TURRET_LAYER: f32 = 2.;
pub const CAMERA_LAYER: f32 = 100.;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(init_background)
                .with_system(init_camera)
        );
    }
}

fn init_background(mut commands: Commands, game_assets: Option<Res<GameAssets>>) {
    let Some(game_assets) = game_assets else { return; };
    commands.spawn(SpriteBundle {
        texture: game_assets.get(SpriteEnum::Background),
        transform: Transform::from_xyz(0., 0., BACKGROUND_LAYER),
        ..default()
    });
}

fn init_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0., 0., CAMERA_LAYER),
            ..default()
        },
        MainCamera
    ));
}
