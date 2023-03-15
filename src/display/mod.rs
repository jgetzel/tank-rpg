use bevy::app::{App, Plugin};
use crate::display::camera::GameCameraPlugin;
use crate::display::sprite_updater::SpriteUpdatePlugin;

pub mod sprite_updater;
pub mod camera;

pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(GameCameraPlugin)
            .add_plugin(SpriteUpdatePlugin);
    }
}
