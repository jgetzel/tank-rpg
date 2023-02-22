use bevy::prelude::{Children, Commands, DespawnRecursiveExt, EventReader, EventWriter, Query, Res, ResMut, Time, Transform, With};
use bevy_rapier2d::dynamics::Velocity;
use bevy::log::info;
use bevy::math::{Quat, Vec3};
use crate::client_input::PlayerInput;
use crate::networking::{Lobby, PlayerJoinEvent};
use crate::object::{Object, SyncedObjects};
use crate::player;
use crate::player::{Player, PlayerSpawnEvent, PlayerTurret};
use crate::player::utils::calc_player_next_velocity;

pub fn player_move(
    mut query: Query<(&mut Velocity, &Player, &PlayerInput)>,
    time: Res<Time>,
) {
    query.iter_mut().for_each(|(mut vel, player, input)| {
        vel.linvel = calc_player_next_velocity(vel.linvel, player, input, time.delta_seconds());
    });
}

pub fn player_turret_rotate(
    player_q: Query<(&Children, &PlayerInput), With<Player>>,
    mut turr_q: Query<(&mut Transform, &mut PlayerTurret)>,
) {
    player_q.iter().for_each(|(children, input)| {
        children.iter().for_each(|&child| {
            if let Ok((mut trans, mut turr)) = turr_q.get_mut(child) {
                turr.direction = input.turret_dir;

                let angle = input.turret_dir.y.atan2(input.turret_dir.x);
                trans.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);
            }
        });
    });
}

pub fn init_player(
    mut join_ev: EventReader<PlayerJoinEvent>,
    mut spawn_ev_w: EventWriter<PlayerSpawnEvent>,
    mut commands: Commands,
    mut lobby: ResMut<Lobby>,
    mut objects: ResMut<SyncedObjects>,
) {
    join_ev.iter().for_each(|ev| {
        info!("Player {} Connected", ev.player_id);
        let player_entity = player::spawn_new_player(&mut commands, ev.player_id, None);
        commands.entity(player_entity).insert(Object { id: ev.object_id });
        lobby.players.insert(ev.player_id, player_entity);
        objects.objects.insert(ev.object_id, player_entity);

        spawn_ev_w.send(PlayerSpawnEvent { player_id: ev.player_id, object_id: ev.object_id });
    });
}
