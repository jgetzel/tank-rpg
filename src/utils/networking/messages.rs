use serde::{Deserialize, Serialize};
use bevy::prelude::{Vec2, Quat, Transform};
use bevy::utils::HashMap;
use crate::asset_loader::components::SpriteEnum;
use crate::simulation::events::{OnPlayerSpawnEvent};
use crate::simulation::ObjectId;
use crate::simulation::PlayerData;
use crate::simulation::server_sim::player::Health;

pub type PlayerId = u64;

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    Unreliable(ServerUnreliableMessage),
    Reliable(ServerReliableMessage),
    Init(ServerInitMessage),
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ServerUnreliableMessage {
    pub player_data: HashMap<PlayerId, PlayerData>,
    pub healths: HashMap<ObjectId, Health>,
    pub object_data: HashMap<ObjectId, PhysicsObjData>,
    pub match_timer: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ServerReliableMessage {
    pub connect_events: Vec<(PlayerId, PlayerData)>,
    pub disconnect_events: Vec<PlayerId>,
    pub player_spawn_events: Vec<OnPlayerSpawnEvent>,
    pub despawn_events: Vec<ObjectId>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ServerInitMessage {
    pub you_connect_event: (PlayerId, PlayerData),
    pub existing_players: Vec<OnPlayerSpawnEvent>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PhysicsObjData {
    pub transform: Transform,
    pub velocity: Vec2,
    pub sprite: Option<SpriteEnum>,
}

pub type TurretRotationData = Quat;


