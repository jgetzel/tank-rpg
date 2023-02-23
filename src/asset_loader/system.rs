use bevy::prelude::{Commands, Res, ResMut, State};
use bevy::asset::{AssetServer, LoadState};
use crate::asset_loader::AppState;
use crate::asset_loader::components::{SPRITE_PATH_MAP};
use crate::asset_loader::resources::{AssetsLoading, SpriteAssets};

pub fn load_sprites(
    mut game_assets: ResMut<SpriteAssets>,
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
) {
    game_assets.set_asset_server(asset_server.clone());
    SPRITE_PATH_MAP.iter().for_each(|(&sprite_enum, &path)| {
       game_assets.insert(sprite_enum, ("sprites/".to_owned() + path).as_str());
    });

    for (_, handle) in game_assets.map.iter() {
        loading.0.push(handle.clone_untyped());
    }
}

pub fn check_assets_loaded(
    mut commands: Commands,
    server: Res<AssetServer>,
    loading: Res<AssetsLoading>,
    mut state: ResMut<State<AppState>>,
) {
    match server.get_group_load_state(loading.0.iter().map(|handle| handle.id)) {
        LoadState::Failed => {}
        LoadState::Loaded => {
            commands.remove_resource::<AssetsLoading>();
            state.set(AppState::InGame).unwrap();
        }
        _ => {}
    };
}
