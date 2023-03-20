use bevy::app::App;
use bevy::prelude::{EventWriter, IntoSystemConfig, Plugin, Res, ResMut, Resource};
use bevy::time::Time;
use crate::ServerSet::ServerUpdate;

pub struct MatchFFAPlugin;

const MATCH_LENGTH_SECS: f32 = 600.;

impl Plugin for MatchFFAPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<OnMatchTimerFinishedEvent>()
            .insert_resource(MatchTimer::new(MATCH_LENGTH_SECS))
            .add_system(match_timer_clock.in_set(ServerUpdate));
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