use bevy::prelude::{Commands, EventWriter, Res, ResMut};
use bevy::asset::{AssetServer, LoadState};
use crate::asset_loader::AssetsLoadedEvent;
use crate::asset_loader::components::{SPRITE_PATH_MAP};
use crate::asset_loader::resources::{AssetsLoading, FontAssets, SpriteAssets};

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

pub fn load_fonts(
    mut font_assets: ResMut<FontAssets>,
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>
) {
    font_assets.load_assets(&asset_server);
    font_assets.handles().into_iter().for_each(|handle| {
        loading.0.push(handle);
    })
}

pub fn check_assets_loaded(
    mut commands: Commands,
    server: Res<AssetServer>,
    loading: Option<Res<AssetsLoading>>,
    mut evt_wr: EventWriter<AssetsLoadedEvent>,
) {
    let Some(loading) = loading else { return; };
    match server.get_group_load_state(loading.0.iter().map(|handle| handle.id)) {
        LoadState::Failed => {}
        LoadState::Loaded => {
            commands.remove_resource::<AssetsLoading>();
            evt_wr.send(AssetsLoadedEvent);
        }
        _ => {}
    };
}
