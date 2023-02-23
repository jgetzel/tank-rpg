use serde::{Deserialize, Serialize};
use bevy::prelude::Component;
use bevy::render::once_cell::sync::Lazy;
use bevy::utils::hashbrown::HashMap;
use crate::asset_loader::components::SpriteEnum::{Background, Bullet, TankGray, TankGrayTurret};

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize, Component, Clone, Copy)]
pub enum SpriteEnum {
    TankGray,
    TankGrayTurret,
    Bullet,
    Background,
}

pub static SPRITE_PATH_MAP: Lazy<HashMap<SpriteEnum, &'static str>> = Lazy::new(||
    HashMap::from([
        (TankGray, "tank_gray.png"),
        (TankGrayTurret, "tank_gray_turret.png"),
        (Bullet, "bullet.png"),
        (Background, "background.png")
    ])
);
