use bevy::app::{App, Plugin};
use bevy::prelude::{Commands, SystemSet};
use crate::asset_loader::AppState;
use crate::prefabs::{default_background, default_camera};

pub const BACKGROUND_LAYER: f32 = -10.;
pub const PLAYER_LAYER: f32 = 0.;
pub const BULLET_LAYER: f32 = 1.;
pub const TURRET_LAYER: f32 = 2.;
pub const CAMERA_LAYER: f32 = 100.;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(init_default)
        );
    }
}

fn init_default(mut commands: Commands) {
    commands.spawn(default_background());
    commands.spawn(default_camera());
}