pub mod server;
pub mod client;
pub mod messages;

use std::collections::HashMap;
use bevy::app::{App, Plugin};
use bevy::prelude::{Entity, Resource};
use crate::networking::messages::{PhysicsObjData, PlayerId};
use crate::object::ObjectId;

pub const PROTOCOL_ID: u64 = 7;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Lobby::default());
        app.add_event::<PlayerJoinEvent>()
            .add_event::<PlayerLeaveEvent>()
            .add_event::<PhysObjUpdateEvent>();
    }
}

#[derive(Debug, Default, Resource)]
pub struct Lobby {
    pub players: HashMap<u64, Entity>,
}

pub struct PlayerJoinEvent {
    pub player_id: PlayerId,
    pub object_id: ObjectId
}

pub struct PlayerLeaveEvent {
    pub player_id: u64,
}

pub struct PhysObjUpdateEvent {
    pub id: ObjectId,
    pub data: PhysicsObjData
}
