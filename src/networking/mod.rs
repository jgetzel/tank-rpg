pub mod server;
pub mod client;
pub mod messages;

use std::collections::HashMap;
use bevy::prelude::{Entity, Resource};

pub const PROTOCOL_ID: u64 = 7;

#[derive(Debug, Default, Resource)]
pub struct Lobby {
    pub players: HashMap<u64, Entity>,
}
