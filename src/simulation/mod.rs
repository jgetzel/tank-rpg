use bevy::app::App;
use bevy::prelude::{Component, Entity, Plugin, Resource};
use bevy::utils::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use crate::utils::networking::Lobby;
use crate::simulation::events::*;
use crate::simulation::client_sim::ClientSimulationPlugin;
use crate::simulation::server_sim::ServerSimulationPlugin;

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
