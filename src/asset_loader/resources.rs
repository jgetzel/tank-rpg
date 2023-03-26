use bevy::asset::{AssetServer, Handle, HandleUntyped};
use bevy::prelude::{Font, Image, Resource};
use std::collections::HashMap;
use crate::asset_loader::components::{FONT_PATH_MAP, FontEnum, SpriteEnum};

#[derive(Default, Resource)]
pub struct SpriteAssets {
    map: HashMap<SpriteEnum, Handle<Image>>,
    asset_server: Option<Box<AssetServer>>,
}

impl SpriteAssets {
    pub fn get(&self, sprite: SpriteEnum) -> Handle<Image> {
        self.map.get(&sprite).unwrap().clone()
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<SpriteEnum, Handle<Image>> {
        self.map.iter()
    }

    pub fn insert(&mut self, sprite: SpriteEnum, path: &str) {
        self.map.insert(sprite, self.asset_server.as_ref().unwrap().load(path));
    }

    pub fn set_asset_server(&mut self, asset_server: AssetServer) {
        self.asset_server = Some(Box::new(asset_server));
    }
}

#[derive(Default, Resource)]
pub struct FontAssets {
    map: HashMap<FontEnum, Handle<Font>>,
}

impl FontAssets {
    #[allow(dead_code)] // TODO Remove once fonts are used
    pub fn get(&self, font: FontEnum) -> Handle<Font> {
        self.map.get(&font).unwrap().clone()
    }

    pub fn load_assets(&mut self, asset_server: &AssetServer) {
        FONT_PATH_MAP.clone().into_iter().for_each(|(font, path)| {
            self.map.insert(font, asset_server.load(format!("fonts/{path}")));
        });
    }

    pub fn handles(&self) -> Vec<HandleUntyped> {
        self.map.values().map(|handle| handle.clone_untyped()).collect()
    }
}

#[derive(Default, Resource)]
pub struct AssetsLoading(pub Vec<HandleUntyped>);
