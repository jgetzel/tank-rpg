use bevy::prelude::{Commands, EventReader, EventWriter, NextState, Res, ResMut, Time};
use crate::AppState;
use crate::simulation::Lobby;
use crate::simulation::server_sim::InGameState;
use crate::simulation::server_sim::match_ffa::{MATCH_LENGTH_SECS, MatchTimer, OnMatchTimerFinishedEvent, OnRestartMatchTimerFinishedEvent, RESTART_WAIT_SECS, RestartMatchTimer};

pub fn init_match_timer_on_enter(mut commands: Commands) {
    commands.insert_resource(MatchTimer::new(MATCH_LENGTH_SECS));
}

pub fn match_timer_clock(
    mut match_timer: ResMut<MatchTimer>,
    time: Res<Time>,
    mut finished_writer: EventWriter<OnMatchTimerFinishedEvent>
) {
    match_timer.time_remaining -= time.delta_seconds();
    if match_timer.time_remaining <= 0. {
        finished_writer.send(OnMatchTimerFinishedEvent);
    }
}

pub fn pause_on_match_finish(
    mut events: EventReader<OnMatchTimerFinishedEvent>,
    mut next_state: ResMut<NextState<InGameState>>
) {
    events.iter().for_each(|_| {
        next_state.set(InGameState::Paused);
    });
}

pub fn start_restart_timer_on_match_finish(
    mut events: EventReader<OnMatchTimerFinishedEvent>,
    mut commands: Commands
) {
    events.iter().for_each(|_| {
        commands.insert_resource(RestartMatchTimer::new(RESTART_WAIT_SECS));
    });
}

pub fn restart_timer_clock(
    mut restart_timer: ResMut<RestartMatchTimer>,
    time: Res<Time>,
    mut finished_writer: EventWriter<OnRestartMatchTimerFinishedEvent>
) {
    restart_timer.time_remaining -= time.delta_seconds();
    if restart_timer.time_remaining <= 0. {
        finished_writer.send(OnRestartMatchTimerFinishedEvent);
    }
}
pub fn new_match_on_restart_timer(
    mut events: EventReader<OnRestartMatchTimerFinishedEvent>,
    mut next_in_game_state: ResMut<NextState<InGameState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    events.iter().for_each(|_| {
        next_in_game_state.set(InGameState::Playing);
        next_app_state.set(AppState::InGame);
    });
}

pub fn clear_player_scores_on_exit(
    mut lobby: ResMut<Lobby>,
) {
    lobby.player_data.values_mut().for_each(|data| {
        data.deaths = 0;
        data.kills = 0;
    });
}