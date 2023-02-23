pub mod resources;
pub mod components;
mod system;

use bevy::app::{App, Plugin};
use bevy::prelude::{SystemSet};
use resources::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Loading,
    InGame,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SpriteAssets::default())
            .insert_resource(AssetsLoading::default())
            .add_system_set(
                SystemSet::on_enter(AppState::Loading)
                    .with_system(system::load_sprites)
            )
            .add_system_set(
                SystemSet::on_update(AppState::Loading)
                    .with_system(system::check_assets_loaded)
            );
    }
}
