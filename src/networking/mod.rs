use bevy::prelude::{ Entity, Resource, Component };
use bevy::utils::HashMap;
use serde::{ Serialize, Deserialize };

#[derive(Debug, Default, Resource)]
pub struct Lobby {
    pub players: HashMap<u64, Entity>,
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    PlayerConnected { id: u64 },
    PlayerDisconnected { id: u64 },
}

