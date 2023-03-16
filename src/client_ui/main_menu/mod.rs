use bevy::app::App;
use bevy::prelude::*;
use crate::AppState;
use crate::simulation::server_sim::despawn_all_entities;

mod systems;
mod ui;


pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ServerIPInput("".into()))
            .insert_resource(ServerPortInput("".into()))
            .add_state::<ConnectState>()
            .add_event::<OnAttemptConnect>()
            .add_event::<OnAttemptHost>()
            .add_system(systems::init.in_schedule(OnEnter(AppState::MainMenu)))
            .add_systems(
                (
                    systems::main_menu_gui.run_if(in_state(ConnectState::NotConnected)),
                    systems::connecting_gui.run_if(in_state(ConnectState::Connecting)),
                    systems::connect_attempt_event_listener,
                    systems::in_game_on_connect
                ).in_set(OnUpdate(AppState::MainMenu))
            )
            .add_system(despawn_all_entities.in_schedule(OnExit(AppState::MainMenu)));
    }
}

#[derive(Resource)]
pub struct ServerIPInput(pub String);

#[derive(Resource)]
pub struct ServerPortInput(pub String);

pub struct OnAttemptConnect;

pub struct OnAttemptHost;

#[derive(States, Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ConnectState {
    #[default]
    NotConnected,
    Connecting,
}
