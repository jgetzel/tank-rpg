use bevy::asset::{AssetServer, Handle, HandleUntyped};
use bevy::prelude::{Image, Resource};
use std::collections::HashMap;
use crate::asset_loader::components::SpriteEnum;

#[derive(Default, Resource)]
pub struct SpriteAssets {
    pub map: HashMap<SpriteEnum, Handle<Image>>,
    asset_server: Option<Box<AssetServer>>,
}

impl SpriteAssets {
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
pub struct AssetsLoading(pub Vec<HandleUntyped>);
