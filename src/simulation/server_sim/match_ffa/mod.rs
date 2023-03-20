mod systems;

use bevy::app::App;
use bevy::prelude::*;
use crate::ServerSet::ServerUpdate;
use systems::*;
use crate::AppState;

pub struct MatchFFAPlugin;

const MATCH_LENGTH_SECS: f32 = 500.;
const RESTART_WAIT_SECS: f32 = 10.;

impl Plugin for MatchFFAPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<OnMatchTimerFinishedEvent>()
            .add_event::<OnRestartMatchTimerFinishedEvent>()
            .add_system(init_match_timer_on_enter.in_schedule(OnEnter(AppState::InGame)))
            .add_system(match_timer_clock.in_set(ServerUpdate)
                .run_if(not(is_match_finished))
                .in_set(OnUpdate(AppState::InGame)))
            .add_systems(
                (
                    pause_on_match_finish,
                    start_restart_timer_on_match_finish,
                    restart_timer_clock.run_if(is_restart_timer_ticking),
                    new_match_on_restart_timer,
                )
            )
            .add_system(clear_player_scores_on_exit.in_schedule(OnExit(AppState::InGame)));
    }
}

#[derive(Resource)]
pub struct MatchTimer {
    pub time_remaining: f32,
    pub match_length: f32,
}

impl MatchTimer {
    pub fn new(length: f32) -> Self {
        Self {
            match_length: length,
            time_remaining: length,
        }
    }
}

#[derive(Resource)]
pub struct RestartMatchTimer {
    pub time_remaining: f32,
}

impl RestartMatchTimer {
    pub fn new(length: f32) -> Self {
        Self {
            time_remaining: length
        }
    }
}

pub struct OnMatchTimerFinishedEvent;

pub struct OnRestartMatchTimerFinishedEvent;

pub fn is_match_finished(match_timer: Option<Res<MatchTimer>>) -> bool {
    let Some(match_timer) = match_timer else { return false; };
    match_timer.time_remaining <= 0.
}

fn is_restart_timer_ticking(restart_timer: Option<Res<RestartMatchTimer>>) -> bool {
    let Some(restart_timer) = restart_timer else { return false; };
    restart_timer.time_remaining > 0.
}