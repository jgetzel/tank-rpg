use std::collections::HashMap;
use bevy::app::{App, Plugin};
use bevy::asset::{AssetServer, Handle, HandleUntyped, LoadState};
use bevy::prelude::{Commands, Component, Image, Res, ResMut, Resource, State, SystemSet};
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
    map: HashMap<SpriteEnum, Handle<Image>>,
    asset_server: Option<Box<AssetServer>>,
}

impl GameAssets {
    pub fn get(&self, sprite: SpriteEnum) -> Handle<Image> {
        self.map.get(&sprite).unwrap().clone()
    }

    pub fn insert(&mut self, sprite: SpriteEnum, path: &str) {
        self.map.insert(sprite, self.asset_server.as_ref().unwrap().load(path));
    }

    pub fn set_asset_server(&mut self, asset_server: AssetServer) {
        self.asset_server = Some(Box::new(asset_server));
    }
}
#[derive(Default, Resource)]
pub struct AssetsLoading(Vec<HandleUntyped>);

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GameAssets::default())
            .insert_resource(AssetsLoading::default())
            .add_system_set(
                SystemSet::on_enter(AppState::Loading)
                    .with_system(load_assets)
            )
            .add_system_set(
                SystemSet::on_update(AppState::Loading)
                    .with_system(check_assets_loaded)
            );
    }
}

fn load_assets(
    mut game_assets: ResMut<GameAssets>,
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
) {
    game_assets.set_asset_server(asset_server.clone());
    game_assets.insert(SpriteEnum::TankGray, "sprites/tank_gray.png");
    game_assets.insert(SpriteEnum::TankGrayTurret, "sprites/tank_gray_turret.png");
    game_assets.insert(SpriteEnum::Bullet,"sprites/bullet.png");
    game_assets.insert(SpriteEnum::Background,"sprites/background.png");

    for (_, handle) in game_assets.map.iter() {
        loading.0.push(handle.clone_untyped());
    }
}

fn check_assets_loaded(
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
