mod systems;
mod client_input;
mod main_menu;
mod debug_ui;
mod leaderboard_ui;
mod health_ui;

pub use crate::player::components::PlayerInput;

use bevy::prelude::*;
use bevy::app::{App, Plugin};
use bevy_quinnet::client::{QuinnetClientPlugin};
use serde::{Deserialize, Serialize};
use crate::networking::client::client_input::ClientInputPlugin;
use crate::networking::client::ClientSet::*;
use crate::networking::client::debug_ui::ClientDebugUI;
use crate::networking::client::health_ui::ClientHealthDisplayPlugin;
use crate::networking::client::leaderboard_ui::ClientLeaderboardUI;
use crate::networking::client::main_menu::MainMenuPlugin;
use crate::networking::client::systems::*;
use crate::networking::messages::{PhysicsObjData, PlayerId};
use crate::networking::PlayerData;
use crate::object::{ObjectId, ObjectSyncPlugin};
use crate::scenes::AppState;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(QuinnetClientPlugin::default())
            .add_plugin(ObjectSyncPlugin)
            .add_plugin(MainMenuPlugin)
            .add_plugin(ClientInputPlugin);

        app
            .add_event::<RecvYouConnectEvent>()
            .add_event::<RecvPlayerConnectEvent>()
            .add_event::<RecvPlayerLeaveEvent>()
            .add_event::<RecvPlayerSpawnEvent>()
            .add_event::<RecvObjectDespawnEvent>()
            .add_event::<RecvHealthUpdateEvent>()
            .add_event::<RecvPhysObjUpdateEvent>()
            .add_event::<RecvTurretUpdateEvent>()
            .add_systems(
                (
                    client_recv.in_set(ClientReceive),
                    client_send.in_set(ClientSend)
                )
            )
            .add_systems(
                (
                    on_you_joined,
                    on_player_join,
                    on_player_leave,
                    on_player_spawn,
                    on_health_update,
                ).in_set(ClientUpdate).before(on_object_despawn)
            )
            .add_system(on_object_despawn.in_set(ClientUpdate))
            .add_system(main_menu_on_load.in_set(OnUpdate(AppState::Loading)));

        #[cfg(debug_assertions)]
        app.add_plugin(ClientDebugUI);

        app.add_plugin(ClientLeaderboardUI)
            .add_plugin(ClientHealthDisplayPlugin);
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

pub struct RecvPlayerSpawnEvent {
    pub player_id: PlayerId,
    pub object_id: ObjectId,
    pub position: Vec2,
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

pub struct RecvTurretUpdateEvent {
    pub parent_id: ObjectId,
    pub rotation: Quat,
}

#[derive(Resource)]
pub struct ClientId(pub PlayerId);

#[allow(clippy::enum_variant_names)]
#[derive(SystemSet, Clone, Hash, Eq, PartialEq, Debug)]
pub enum ClientSet {
    ClientReceive,
    ClientUpdate,
    ClientSend,
}

#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    InputMessage {
        input: PlayerInput
    }
}
