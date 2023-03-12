use crate::networking::messages::PlayerId;
use crate::object::ObjectId;

pub struct OnObjectDespawnEvent {
    pub id: ObjectId,
}

pub struct OnPlayerConnectEvent {
    pub player_id: PlayerId
}

pub struct OnPlayerSpawnEvent {
    pub player_id: PlayerId,
    pub object_id: ObjectId
}
