
use bevy::app::App;
use bevy::prelude::*;
use crate::AppState;
use crate::utils::prefabs::{default_background, default_camera, spawn_point};

pub struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(init_default.in_schedule(OnEnter(AppState::InGame)));
    }
}

pub fn init_default(mut commands: Commands) {
    commands.spawn(default_background());
    commands.spawn(default_camera());
    commands.spawn(spawn_point([-500.0, 0.].into()));
    commands.spawn(spawn_point([500.0, 0.].into()));
    commands.spawn(spawn_point([0., -500.0].into()));
    commands.spawn(spawn_point([0., 500.0].into()));
}
