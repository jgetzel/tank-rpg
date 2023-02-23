mod systems;

use bevy::app::{App, Plugin};
use bevy::prelude::{Component, SystemSet};
use bevy::ecs::system::Resource;
use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};
use crate::networking::client::ClientEventSysLabel::ClientSend;
use crate::player::components::PlayerInput;

pub struct ClientInputPlugin;

impl Plugin for ClientInputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerInput::default())
            .add_system_set(
                SystemSet::new().before(ClientSend)
                    .with_system(systems::keyboard_events)
                    .with_system(systems::mouse_position)
                    .with_system(systems::mouse_click)
            );
    }
}
