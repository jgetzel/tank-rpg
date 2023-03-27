use bevy::app::App;
use bevy::prelude::{Component, default, Entity, Plugin, Resource};
use bevy::utils::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use serde::{Deserialize, Serialize};
use crate::simulation::events::*;
use crate::simulation::client_sim::ClientSimulationPlugin;
use crate::simulation::server_sim::ServerSimulationPlugin;
use crate::utils::networking::messages::PlayerId;

pub mod events;
pub mod client_sim;
pub mod server_sim;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(ServerSimulationPlugin)
            .add_plugin(ClientSimulationPlugin);

        app
            .insert_resource(Lobby::default())
            .insert_resource(SyncedObjects::default());

        app
            .add_event::<OnObjectDespawnEvent>()
            .add_event::<OnPlayerSpawnEvent>()
            .add_event::<OnPlayerDisconnectEvent>()
            .add_event::<OnPlayerConnectEvent>();

    }
}

#[derive(Debug, Default, Resource)]
pub struct SyncedObjects {
    pub objects: HashMap<ObjectId, Entity>,
}

pub type ObjectId = u64;

static COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Component, Copy, Clone)]
pub struct Object {
    pub id: ObjectId
}

impl Object {
    pub fn new() -> Self {
        Object {
            id: COUNTER.fetch_add(1, Ordering::Relaxed)
        }
    }
}

impl Default for Object {
    fn default() -> Self {
        Object::new()
    }
}

#[derive(Debug, Default, Resource)]
pub struct Lobby {
    pub player_data: HashMap<PlayerId, PlayerData>,
}

impl Lobby {
    pub fn update_object_id(&mut self, player_id: PlayerId, object_id: ObjectId) -> Result<(), String> {
        if let Some(mut data) = self.player_data.get_mut(&player_id) {
            if data.object_id.is_some() {
                return Err(format!("Attempted to update object ID for Player {}, \
                but they were already connected to another object!", player_id));
            }
            data.object_id = Some(object_id);
            Ok(())
        }
        else {
            self.player_data.insert(player_id, PlayerData::new(object_id));
            Ok(())
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerData {
    pub object_id: Option<ObjectId>,
    pub kills: u32,
    pub deaths: u32,
}

impl PlayerData {
    pub fn new(object_id: ObjectId) -> Self {
        PlayerData {
            object_id: Some(object_id),
            ..default()
        }
    }
}
