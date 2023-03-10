pub mod server;
pub mod client;
pub mod messages;

use std::collections::HashMap;
use bevy::app::{App, Plugin};
use bevy::prelude::{Entity, Quat, Res, Resource};
use bevy_quinnet::client::Client;
use crate::networking::messages::{PhysicsObjData, PlayerId};
use crate::object::ObjectId;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Lobby::default());
        app
            .add_event::<PlayerConnectEvent>()
            .add_event::<PlayerLeaveEvent>()
            .add_event::<ObjectDespawnEvent>()
            .add_event::<PhysObjUpdateEvent>()
            .add_event::<TurretUpdateEvent>();
    }
}

pub fn is_client_exe(client: Option<Res<Client>>) -> bool {
    client.is_some()
}

#[derive(Debug, Default, Resource)]
pub struct Lobby {
    pub players: HashMap<PlayerId, Entity>,
}

pub struct PlayerConnectEvent {
    pub player_id: PlayerId,
    pub object_id: ObjectId
}

pub struct PlayerLeaveEvent {
    pub player_id: PlayerId,
}

pub struct ObjectDespawnEvent {
    pub object_id: ObjectId
}

pub struct PhysObjUpdateEvent {
    pub id: ObjectId,
    pub data: PhysicsObjData
}

pub struct TurretUpdateEvent {
    pub parent_id: ObjectId,
    pub rotation: Quat,
}
