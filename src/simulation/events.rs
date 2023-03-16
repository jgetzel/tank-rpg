use bevy::math::Vec2;
use crate::utils::networking::messages::PlayerId;
use crate::simulation::ObjectId;

pub struct OnObjectDespawnEvent {
    pub id: ObjectId,
}

pub struct OnPlayerConnectEvent {
    pub player_id: PlayerId
}

pub struct OnPlayerSpawnEvent {
    pub player_id: PlayerId,
    pub object_id: ObjectId,
    pub position: Vec2,
}

pub struct OnRespawnTimerFinish {
    pub player_id: PlayerId,
}
