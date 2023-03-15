use bevy::prelude::{Component, Entity, Reflect, Resource};
use bevy::math::Vec2;
use serde::{Deserialize, Serialize};
use crate::client_networking::ClientInput;
use crate::utils::messages::PlayerId;

#[derive(Component)]
pub struct You;

const DEFAULT_HEALTH: f32 = 20.0;

#[derive(Component)]
pub struct Health {
    pub max_health: f32,
    pub health: f32,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            max_health: DEFAULT_HEALTH,
            health: DEFAULT_HEALTH,
        }
    }
}

#[derive(Component, Clone)]
pub struct Player {
    pub id: PlayerId,
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

impl From<ClientInput> for PlayerInput {
    fn from(client_input: ClientInput) -> Self {
        PlayerInput {
            movement: client_input.movement,
            mouse_pos: client_input.mouse_pos,
            fire_bullet: client_input.fire_bullet,
        }
    }
}
