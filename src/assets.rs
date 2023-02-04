use std::collections::HashMap;
use bevy::asset::{AssetServer, Handle, HandleUntyped, LoadState};
use bevy::prelude::{Commands, Component, Image, Res, ResMut, Resource, State};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Loading,
    InGame,
}

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize, Component, Clone, Copy)]
pub enum SpriteEnum {
    TankGray,
    TankGrayTurret,
    Bullet,
    Background,
}

#[derive(Default, Resource)]
pub struct GameAssets {
    pub map: HashMap<SpriteEnum, Handle<Image>>
}

#[derive(Default, Resource)]
pub struct AssetsLoading(Vec<HandleUntyped>);

pub fn load_assets(
    mut game_assets: ResMut<GameAssets>,
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
) {

    game_assets.map.insert(SpriteEnum::TankGray, asset_server.load("sprites/tank_gray.png"));
    game_assets.map.insert(SpriteEnum::TankGray, asset_server.load("sprites/tank_gray_turret.png"));
    game_assets.map.insert(SpriteEnum::Bullet, asset_server.load("sprites/bullet.png"));
    game_assets.map.insert(SpriteEnum::Background, asset_server.load("sprites/background.png"));

    for (_, handle) in game_assets.map.iter() {
        loading.0.push(handle.clone_untyped());
    }
}

pub fn check_assets_loaded(
    mut commands: Commands,
    server: Res<AssetServer>,
    loading: Res<AssetsLoading>,
    mut state: ResMut<State<AppState>>
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
