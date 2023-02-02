use std::iter::zip;
use bevy::prelude::{Camera, Camera2dBundle, Commands, Query, Res, Transform, With, Without, Component};
use bevy::time::Time;
use bevy::utils::default;
use crate::environment::CAMERA_LAYER;
use crate::player::{Player, You};

static CAMERA_SMOOTHING: f32 = 2.;

#[derive(Component)]
pub struct MainCamera;

pub fn init_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0., 0., CAMERA_LAYER),
            ..default()
        },
        MainCamera
    ));
}

pub fn camera_move(
    player_query: Query<&Transform, With<You>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<You>)>,
    time: Res<Time>,
) {
    for (player_trans, mut cam_trans) in zip(
        player_query.iter(),
        camera_query.iter_mut())
    {
        cam_trans.translation = cam_trans.translation.lerp(
            player_trans.translation.truncate().extend(CAMERA_LAYER),
            CAMERA_SMOOTHING * time.delta_seconds(),
        );
    }
}