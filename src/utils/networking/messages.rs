use serde::{Deserialize, Serialize};
use bevy::prelude::{Vec2, Quat, Transform};
use bevy::utils::HashMap;
use crate::asset_loader::components::SpriteEnum;
use crate::simulation::ObjectId;
use crate::simulation::PlayerData;

pub type PlayerId = u64;

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    YouConnected { player_id: PlayerId },
    PlayerConnected { player_id: PlayerId, data: PlayerData },
    PlayerDisconnected { player_id: PlayerId },
    ObjectDespawn { object_id: ObjectId },
    PlayerSpawn { player_id: PlayerId, object_id: ObjectId, position: Vec2 },
    MatchTimerMsg { time_remaining: f32 },
    PhysObjUpdate { objects: HashMap<ObjectId, PhysicsObjData> },
    PlayerDataUpdate { player_id: PlayerId, data: PlayerData }, //TODO find a better way to update K/D count
    HealthUpdate { object_id: ObjectId, health: f32, max_health: f32 },
    TurretRotationUpdate { turrets: HashMap<ObjectId, TurretRotationData> } //TODO find a better way
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PhysicsObjData {
    pub transform: Transform,
    pub velocity: Vec2,
    pub sprite: Option<SpriteEnum>,
}

pub type TurretRotationData = Quat;


