use serde::{Deserialize, Serialize};
use bevy::prelude::{Vec3, Vec2, Quat};
use bevy::utils::HashMap;
use crate::asset_loader::components::SpriteEnum;
use crate::object::ObjectId;

pub type PlayerId = u64;

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    YouConnected { player_id: PlayerId },
    PlayerConnected { player_id: PlayerId, object_id: ObjectId },
    PlayerDisconnected { player_id: PlayerId },
    ObjectDespawn { object_id: ObjectId },
    PhysObjUpdate { objects: HashMap<ObjectId, PhysicsObjData> },
    TurretRotationUpdate { turrets: HashMap<ObjectId, TurretRotationData> } //TODO find a better way
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PhysicsObjData {
    pub translation: Vec3,
    pub velocity: Vec2,
    pub sprite: SpriteEnum,
}

pub type TurretRotationData = Quat;


