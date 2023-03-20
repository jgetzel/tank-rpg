use bevy::app::App;
use bevy::prelude::{EventReader, EventWriter, IntoSystemConfig, NextState, not, Plugin, Res, ResMut, Resource};
use bevy::time::Time;
use crate::ServerSet::ServerUpdate;
use crate::simulation::server_sim::InGameState;

pub struct MatchFFAPlugin;

const MATCH_LENGTH_SECS: f32 = 10.;

impl Plugin for MatchFFAPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<OnMatchTimerFinishedEvent>()
            .insert_resource(MatchTimer::new(MATCH_LENGTH_SECS))
            .add_system(match_timer_clock.in_set(ServerUpdate).run_if(not(is_match_finished)))
            .add_system(pause_on_match_finish);
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

pub struct OnMatchTimerFinishedEvent;

pub fn is_match_finished(match_timer: Option<Res<MatchTimer>>) -> bool {
    let Some(match_timer) = match_timer else { return false; };
    match_timer.time_remaining <= 0.
}

fn match_timer_clock(
    mut match_timer: ResMut<MatchTimer>,
    time: Res<Time>,
    mut finished_writer: EventWriter<OnMatchTimerFinishedEvent>
) {
    match_timer.time_remaining -= time.delta_seconds();
    if match_timer.time_remaining <= 0. {
        finished_writer.send(OnMatchTimerFinishedEvent);
    }
}

fn pause_on_match_finish(
    mut events: EventReader<OnMatchTimerFinishedEvent>,
    mut next_state: ResMut<NextState<InGameState>>
) {
    events.iter().for_each(|_| {
        next_state.set(InGameState::Paused);
    });
}