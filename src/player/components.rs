use bevy::prelude::{Component, Entity, Reflect, Resource};
use bevy::math::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct You;

#[derive(Component, Clone)]
pub struct Player {
    pub id: u64,
    pub accel: f32,
    pub max_speed: f32,
    pub friction: f32,
}

impl Player {
    pub fn new(id: u64) -> Self {
        Player {
            id,
            accel: 2400.,
            max_speed: 300.,
            friction: 500.,
        }
    }
}

#[derive(Component)]
pub struct PlayerTurret {
    pub owner: Option<Entity>,
    pub direction: Vec2,
    pub bullet_speed: f32,
}

impl Default for PlayerTurret {
    fn default() -> Self {
        PlayerTurret {
            owner: None,
            direction: Vec2::default(),
            bullet_speed: 600.,
        }
    }
}

#[derive(Default, Component, Resource, Reflect, bevy::reflect::FromReflect, Debug, Clone,
Serialize, Deserialize)]
pub struct PlayerInput {
    pub movement: Vec2,
    pub mouse_pos: Vec2,
    pub fire_bullet: bool,
}
