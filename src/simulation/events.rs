use bevy::math::Vec2;
use serde::{Deserialize, Serialize};
use crate::utils::networking::messages::PlayerId;
use crate::simulation::ObjectId;

pub struct OnObjectDespawnEvent {
    pub id: ObjectId,
}

pub struct OnPlayerConnectEvent {
    pub player_id: PlayerId
}

pub struct OnPlayerDisconnectEvent {
    pub player_id: PlayerId
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OnPlayerSpawnEvent {
    pub player_id: PlayerId,
    pub object_id: ObjectId,
    pub turret_object_ids: Vec<ObjectId>,
    pub position: Vec2,
}

pub struct OnRespawnTimerFinish {
    pub player_id: PlayerId,
}
