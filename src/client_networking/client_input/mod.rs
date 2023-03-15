mod systems;

use bevy::app::{App, Plugin};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::client_networking::client_input::systems::*;
use crate::ClientSet::ClientSend;

pub struct ClientInputPlugin;

impl Plugin for ClientInputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClientInput::default())
            .add_systems(
                (
                    keyboard_events,
                    mouse_position,
                    mouse_click
                ).before(ClientSend)
            );
    }
}

#[derive(Resource, Default, Clone, Serialize, Deserialize)]
pub struct ClientInput {
    pub movement: Vec2,
    pub mouse_pos: Vec2,
    pub fire_bullet: bool,
}