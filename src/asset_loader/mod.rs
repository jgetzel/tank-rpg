pub mod resources;
pub mod components;
mod system;

use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemAppConfigs, IntoSystemConfig, OnEnter, OnUpdate};
use resources::*;
use crate::asset_loader::components::SpriteEnum;
use crate::AppState;
use crate::asset_loader::system::{check_assets_loaded, load_fonts, load_sprites};

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SpriteAssets::default())
            .insert_resource(FontAssets::default())
            .register_type::<SpriteEnum>()
            .insert_resource(AssetsLoading::default())
            .add_event::<AssetsLoadedEvent>()
            .add_systems(
                (
                    load_sprites,
                    load_fonts
                ).in_schedule(OnEnter(AppState::Loading))
            )
            .add_system(check_assets_loaded.in_set(OnUpdate(AppState::Loading)));
    }
}

pub struct AssetsLoadedEvent;
