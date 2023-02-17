use bevy::prelude::{Commands, EventReader, Query, Res, ResMut, Time, Transform};
use bevy_rapier2d::dynamics::Velocity;
use bevy::log::info;
use bevy::math::{Quat, Vec3};
use crate::input_helper::PlayerInput;
use crate::networking::{Lobby, PlayerJoinEvent};
use crate::object::SyncedObjects;
use crate::player;
use crate::player::{Player, PlayerTurret};

pub fn player_move(
    mut query: Query<(&mut Velocity, &Player, &PlayerInput)>,
    time: Res<Time>,
) {
    for (mut velocity, player, input) in query.iter_mut() {
        let new_velocity = velocity.linvel + (player.accel * input.movement * time.delta_seconds());
        velocity.linvel =
            if [player.max_speed, velocity.linvel.length()].iter().all(|v| new_velocity.length() > *v) {
                new_velocity.clamp_length_max(player.max_speed)
            } else {
                new_velocity
            };
    }
}

pub fn player_turret_rotate(
    input: Res<PlayerInput>,
    mut query: Query<(&mut Transform, &mut PlayerTurret)>,
) {
    for (mut trans, mut turret) in query.iter_mut() {
        turret.direction = input.turret_dir;

        let angle = input.turret_dir.y.atan2(input.turret_dir.x);
        trans.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);
    }
}

pub fn init_player(
    mut join_ev: EventReader<PlayerJoinEvent>,
    mut commands: Commands,
    mut lobby: ResMut<Lobby>,
    mut objects: ResMut<SyncedObjects>
) {
    for ev in join_ev.iter() {
        info!("Player {} Connected", ev.player_id);
        let player_entity = player::spawn_new_player(&mut commands, ev.player_id, None);
        lobby.players.insert(ev.player_id, player_entity);
        objects.objects.insert(ev.object_id, player_entity);
    }
}
