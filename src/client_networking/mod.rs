mod systems;
mod client_input;

pub use crate::client_networking::client_input::ClientInput;

use bevy::prelude::*;
use bevy::app::{App, Plugin};
use bevy_quinnet::client::QuinnetClientPlugin;
use serde::{Deserialize, Serialize};
use crate::client_networking::client_input::ClientInputPlugin;
use crate::ClientSet::*;
use crate::client_networking::systems::*;
use crate::utils::networking::messages::{PhysicsObjData, PlayerId};
use crate::simulation::PlayerData;
use crate::simulation::ObjectId;

pub struct ClientNetworkingPlugin;

impl Plugin for ClientNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(QuinnetClientPlugin::default())
            .add_plugin(ClientInputPlugin);

        app
            .add_event::<RecvYouConnectEvent>()
            .add_event::<RecvPlayerConnectEvent>()
            .add_event::<RecvPlayerLeaveEvent>()
            .add_event::<RecvPlayerSpawnEvent>()
            .add_event::<RecvMatchTimeEvent>()
            .add_event::<RecvObjectDespawnEvent>()
            .add_event::<RecvHealthUpdateEvent>()
            .add_event::<RecvPhysObjUpdateEvent>()
            .add_event::<RecvPlayerDataUpdateEvent>()
            .add_event::<RecvTurretUpdateEvent>()
            .add_systems(
                (
                    client_recv.in_set(ClientReceive),
                    client_send.in_set(ClientSend)
                )
            );
    }
}

pub struct RecvYouConnectEvent {
    pub player_id: PlayerId,
}

pub struct RecvPlayerConnectEvent {
    pub player_id: PlayerId,
    pub data: PlayerData,
}

pub struct RecvPlayerLeaveEvent {
    pub player_id: PlayerId,
}

#[derive(Clone)]
pub struct RecvPlayerSpawnEvent {
    pub player_id: PlayerId,
    pub object_id: ObjectId,
    pub position: Vec2,
}

pub struct RecvMatchTimeEvent {
    pub time_remaining: f32,
}

pub struct RecvObjectDespawnEvent {
    pub object_id: ObjectId,
}

pub struct RecvHealthUpdateEvent {
    pub object_id: ObjectId,
    pub health: f32,
    pub max_health: f32,
}

pub struct RecvPhysObjUpdateEvent {
    pub id: ObjectId,
    pub data: PhysicsObjData,
}

pub struct RecvPlayerDataUpdateEvent {
    pub id: PlayerId,
    pub data: PlayerData
}

pub struct RecvTurretUpdateEvent {
    pub parent_id: ObjectId,
    pub rotation: Quat,
}

#[derive(Resource)]
pub struct ClientId(pub PlayerId);

#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    InputMessage {
        input: ClientInput
    }
}
