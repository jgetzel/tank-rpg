use std::iter::zip;
use bevy::prelude::{Camera, Camera2dBundle, Commands, Component, debug, EventReader, Query, Res, Transform, With, Without};
use bevy::time::Time;
use bevy::utils::default;
use bevy_renet::renet::RenetClient;
use crate::environment::CAMERA_LAYER;
use crate::networking::client::PlayerJoinEvent;
use crate::networking::Lobby;
use crate::player::You;

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

pub fn you_tag_adder(
    mut join_event: EventReader<PlayerJoinEvent>,
    mut commands: Commands,
    client: Res<RenetClient>,
    lobby: Res<Lobby>
) {
    for ev in join_event.iter() {
        if ev.player_id == client.client_id() {
            if let Some(&player_entity) = lobby.players.get(&ev.player_id) {
                commands.entity(player_entity).insert(You);
                debug!("'You' tag added for Player {}", ev.player_id);
            }
        }
    }
}
