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
    pub player_data: HashMap<PlayerId, PlayerData>,
}

#[derive(Debug, Clone)]
pub struct PlayerData {
    pub entity: Option<Entity>,
}

impl Lobby {
    pub fn get_entity(&self, id: &PlayerId) -> Option<Entity> {
        if let Some(data) = self.player_data.get(id) {
            data.entity
        } else { None }
    }

    pub fn get_data(&self, id: &PlayerId) -> Option<&PlayerData> {
        self.player_data.get(id)
    }

    pub fn insert_data<'a>(&mut self, id: PlayerId, data: PlayerData) -> Result<(), &'static str> {
        let old_data = self.player_data.insert(id, data);

        if old_data.is_some() {
            return Err("Tried to insert new player data where data was already assigned!");
        }

        Ok(())
    }

    pub fn insert_entity(&mut self, id: PlayerId, entity: Entity) -> Result<(), &'static str> {
        let mut data = self.player_data.get_mut(&id);
        return if let Some(data) = data {
            if data.entity.is_some() {
                return Err("Player Entity already exists!");
            }
            data.entity = Some(entity);
            Ok(())
        } else {
            Err("No player data to insert entity into!")
        };
    }

    pub fn remove_entity(&mut self, id: &PlayerId) {
        if let Some(mut data) = self.player_data.get_mut(&id) {
            data.entity = None;
        }
    }
}

pub struct PlayerConnectEvent {
    pub player_id: PlayerId,
    pub object_id: ObjectId,
}

pub struct PlayerLeaveEvent {
    pub player_id: PlayerId,
}

pub struct ObjectDespawnEvent {
    pub object_id: ObjectId,
}

pub struct PhysObjUpdateEvent {
    pub id: ObjectId,
    pub data: PhysicsObjData,
}

pub struct TurretUpdateEvent {
    pub parent_id: ObjectId,
    pub rotation: Quat,
}
