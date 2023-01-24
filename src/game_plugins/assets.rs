use bevy::asset::{AssetServer, Handle, HandleUntyped, LoadState};
use bevy::prelude::{Commands, Image, Res, ResMut, Resource, State};
use crate::AppState;

#[derive(Default, Resource)]
pub struct GameAssets {
    pub tank_gray: Handle<Image>,
    pub tank_gray_turret: Handle<Image>,
    pub bullet: Handle<Image>,
    pub background: Handle<Image>,
}

#[derive(Default, Resource)]
pub struct AssetsLoading(Vec<HandleUntyped>);

pub fn load_assets(
    mut game_assets: ResMut<GameAssets>,
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
) {
    game_assets.tank_gray = asset_server.load("sprites/tank_gray.png");
    game_assets.tank_gray_turret = asset_server.load("sprites/tank_gray_turret.png");
    game_assets.bullet = asset_server.load("sprites/bullet.png");
    game_assets.background = asset_server.load("sprites/background.png");

    loading.0.push(game_assets.tank_gray.clone_untyped());
    loading.0.push(game_assets.tank_gray_turret.clone_untyped());
    loading.0.push(game_assets.bullet.clone_untyped());
    loading.0.push(game_assets.background.clone_untyped());
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
