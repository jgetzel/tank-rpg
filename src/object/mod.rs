mod systems;
pub mod components;

pub use components::*;
use std::sync::atomic::AtomicU64;
use bevy::app::App;
use bevy::prelude::{Entity, Plugin, Resource};
use bevy::utils::HashMap;

static COUNTER: AtomicU64 = AtomicU64::new(0);

pub type ObjectId = u64;

#[derive(Debug, Default, Resource)]
pub struct SyncedObjects {
    pub objects: HashMap<ObjectId, Entity>,
}


pub struct ObjectSyncPlugin;

impl Plugin for ObjectSyncPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(systems::phys_obj_updater);
    }
}