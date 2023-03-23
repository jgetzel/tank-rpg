use serde::{Deserialize, Serialize};
use bevy::prelude::Component;
use bevy::reflect::Reflect;
use bevy::ecs::reflect::{ReflectComponent};
use bevy::render::once_cell::sync::Lazy;
use bevy::utils::hashbrown::HashMap;
use SpriteEnum::*;
use crate::asset_loader::components::FontEnum::Anta;

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize, Component, Reflect, Default, Clone, Copy)]
#[reflect(Component)]
pub enum SpriteEnum {
    #[default]
    TankDefault,
    TankDefaultTurret,
    TankClassic,
    TankClassicTurret,
    TreeTrunk1,
    TreeLeaves1,
    Bullet,
    Background,
}

pub static SPRITE_PATH_MAP: Lazy<HashMap<SpriteEnum, &str>> = Lazy::new(||
    HashMap::from([
        (TankDefault, "tank_default.png"),
        (TankDefaultTurret, "tank_default_turret.png"),
        (TankClassic, "tank_classic.png"),
        (TankClassicTurret, "tank_classic_turret.png"),
        (TreeTrunk1, "tree_trunk_1.png"),
        (TreeLeaves1, "tree_leaves_1.png"),
        (Bullet, "bullet.png"),
        (Background, "background.png")
    ])
);

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum FontEnum {
    Anta
}

pub static FONT_PATH_MAP: Lazy<HashMap<FontEnum, &str>> = Lazy::new(||
    HashMap::from([
        (Anta, "Anta-Regular.ttf")
    ])
);