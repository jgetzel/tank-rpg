use std::iter::zip;
use bevy::app::{App, Plugin};
use bevy::prelude::{Camera, Commands, Component, debug, EventReader, Query, Res, Transform, With, Without};
use bevy::time::Time;
use bevy_renet::renet::RenetClient;
use crate::environment::CAMERA_LAYER;
use crate::networking::Lobby;
use crate::player::components::You;
use crate::player::PlayerSpawnEvent;

static CAMERA_SMOOTHING: f32 = 2.;

#[derive(Component)]
pub struct MainCamera;

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(camera_move)
            .add_system(you_tag_adder);
    }
}

fn camera_move(
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

fn you_tag_adder(
    mut spawn_event: EventReader<PlayerSpawnEvent>,
    mut commands: Commands,
    client: Option<Res<RenetClient>>,
    lobby: Res<Lobby>,
) {
    let Some(client) = client else { return; };
    for ev in spawn_event.iter() {
        if ev.player_id == client.client_id() {
            if let Some(&player_entity) = lobby.players.get(&ev.player_id) {
                commands.entity(player_entity).insert(You);
                debug!("'You' tag added for Player {}", ev.player_id);
            }
        }
    }
}
