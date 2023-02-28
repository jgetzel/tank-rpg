pub mod resources;
pub mod components;
mod system;

use bevy::app::{App, Plugin};
use bevy::prelude::SystemSet;
use resources::*;
use crate::asset_loader::components::SpriteEnum;
use crate::AppState;

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SpriteAssets::default())
            .insert_resource(FontAssets::default())
            .register_type::<SpriteEnum>()
            .insert_resource(AssetsLoading::default())
            .add_event::<AssetsLoadedEvent>()
            .add_system_set(
                SystemSet::on_enter(AppState::Loading)
                    .with_system(system::load_sprites)
                    .with_system(system::load_fonts)
            )
            .add_system_set(
                SystemSet::on_update(AppState::Loading)
                    .with_system(system::check_assets_loaded)
            );
    }
}

pub struct AssetsLoadedEvent;
