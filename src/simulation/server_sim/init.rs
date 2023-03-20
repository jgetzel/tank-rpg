
use bevy::app::App;
use bevy::prelude::*;
use crate::AppState;
use crate::simulation::server_sim::despawn_all_entities;
use crate::utils::prefabs::{default_background, default_camera, spawn_point};

pub struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<OnInitEvent>()
            .add_system(init_default.in_schedule(OnEnter(AppState::InGame)))
            .add_system(despawn_all_entities.in_schedule(OnExit(AppState::InGame)));
    }
}

pub fn init_default(
    mut commands: Commands,
    mut init_writer: EventWriter<OnInitEvent>
) {
    commands.spawn(default_background());
    commands.spawn(default_camera());
    commands.spawn(spawn_point([-500.0, 0.].into()));
    commands.spawn(spawn_point([500.0, 0.].into()));
    commands.spawn(spawn_point([0., -500.0].into()));
    commands.spawn(spawn_point([0., 500.0].into()));

    init_writer.send(OnInitEvent);
}

pub struct OnInitEvent;