mod systems;

use bevy::app::{App, Plugin};
use bevy::prelude::*;
use crate::networking::client::client_input::systems::{keyboard_events, mouse_click, mouse_position};
use crate::networking::client::ClientSet::ClientSend;
use crate::player::components::PlayerInput;

pub struct ClientInputPlugin;

impl Plugin for ClientInputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerInput::default())
            .add_systems(
                (
                    keyboard_events,
                    mouse_position,
                    mouse_click
                ).before(ClientSend)
            );
    }
}
