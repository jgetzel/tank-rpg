use serde::{Deserialize, Serialize};
use bevy::prelude::{ Vec3, Vec2};
use bevy::utils::HashMap;
use crate::assets::SpriteEnum;
use crate::object::ObjectId;

pub type PlayerId = u64;

#[derive(Debug, Serialize, Deserialize)]
pub enum ReliableMessages {
    PlayerConnected { player_id: PlayerId, object_id: ObjectId },
    PlayerDisconnected { player_id: PlayerId },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UnreliableMessages {
    PhysObjUpdate { objects: HashMap<ObjectId, PhysicsObjData> }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhysicsObjData {
    pub translation: Vec3,
    pub velocity: Vec2,
    pub sprite: SpriteEnum,
}


