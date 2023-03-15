use bevy::prelude::{EventReader, EventWriter, Res, ResMut, Time};
use crate::simulation::events::OnRespawnTimerFinish;
use crate::simulation::server_sim::player::OnPlayerDeathEvent;
use crate::simulation::server_sim::respawn::RespawnTimer;

pub fn start_respawn_timer_on_death(
    mut death_reader: EventReader<OnPlayerDeathEvent>,
    mut respawn_timer: ResMut<RespawnTimer>,
) {
    death_reader.iter().for_each(|e| {
        respawn_timer.map.insert(e.player_id, 5.0);
    });
}

pub fn run_respawn_timer(
    mut respawn_timer: ResMut<RespawnTimer>,
    time: Res<Time>,
) {
    respawn_timer.map.values_mut().for_each(|v| {
        *v -= time.delta_seconds();
    });
}

pub fn dispatch_respawn_on_countdown(
    mut respawn_timer: ResMut<RespawnTimer>,
    mut respawn_event_writer: EventWriter<OnRespawnTimerFinish>,
) {
    respawn_timer.map.iter().for_each(|(player_id, time)| {
        if *time <= 0. {
            respawn_event_writer.send(OnRespawnTimerFinish { player_id: *player_id });
        }
    });

    respawn_timer.map = respawn_timer.map.clone().into_iter().filter(|(_, time)| *time > 0.).collect();
}
