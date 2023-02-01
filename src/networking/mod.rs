pub mod server;
pub mod client;

use bevy::prelude::{Component, Entity, Resource};
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

pub const PROTOCOL_ID: u64 = 7;

#[derive(Debug, Default, Resource)]
pub struct Lobby {
    pub players: HashMap<u64, Entity>,
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    PlayerConnected { id: u64 },
    PlayerDisconnected { id: u64 },
}
