use std::net::{SocketAddr};
use std::str::FromStr;
use bevy::app::App;
use bevy_egui::{EguiContext};
use bevy::prelude::{Commands, EventReader, EventWriter, Plugin, Res, ResMut, Resource, State, SystemSet};
use bevy::utils::default;
use bevy_editor_pls::egui;
use bevy_editor_pls::egui::Align;
use bevy_editor_pls::egui::style::Margin;
use bevy_egui::egui::{Align2, Color32};
use bevy_egui::egui::epaint::Shadow;
use bevy_renet::renet::RenetClient;
use crate::networking::client::new_client;
use crate::prefabs::default_camera;
use crate::scenes::{AppState, despawn_all_entities};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ConnectState {
    NotConnected,
    Connecting,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ServerIPInput("".into()))
            .add_state(ConnectState::NotConnected)
            .add_event::<AttemptConnectEvent>()
            .add_system_set(
                SystemSet::on_enter(AppState::MainMenu)
                    .with_system(init)
            ).add_system_set(
            SystemSet::on_update(AppState::MainMenu)
                .with_system(main_menu_gui)
                .with_system(connect_attempt_event_listener)
                .with_system(in_game_on_connect)
        ).add_system_set(
            SystemSet::on_exit(AppState::MainMenu)
                .with_system(despawn_all_entities)
        );
    }
}

#[derive(Resource)]
struct ServerIPInput(pub String);

struct AttemptConnectEvent;

fn init(mut commands: Commands) {
    commands.spawn(default_camera());
}

fn main_menu_gui(
    mut egui_ctx: ResMut<EguiContext>,
    mut server_ip_string: ResMut<ServerIPInput>,
    mut connect_writer: EventWriter<AttemptConnectEvent>
) {
    let panel_frame = egui::Frame {
        inner_margin: Margin::from(10.),
        rounding: 5.0.into(),
        shadow: Shadow {
            extrusion: 4.,
            color: Color32::from_black_alpha(50),
        },
        fill: egui_ctx.ctx_mut().style().visuals.window_fill(),
        ..default()
    };

    egui::Window::new("Enter Server IP")
        .frame(panel_frame)
        .fixed_size([300., 50.])
        .anchor(Align2([Align::Center, Align::Center]), [0., 0.])
        .collapsible(false)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.horizontal_centered(|ui| {
                ui.text_edit_singleline(&mut (server_ip_string.0));
                if ui.add_enabled(
                    SocketAddr::from_str(server_ip_string.0.as_str()).is_ok(),
                    egui::Button::new("Connect!"),
                ).clicked() {
                    connect_writer.send(AttemptConnectEvent);
                }
            });
        });
}

fn connect_attempt_event_listener(
    mut evt: EventReader<AttemptConnectEvent>,
    server_address: Res<ServerIPInput>,
    mut state: ResMut<State<ConnectState>>,
    mut commands: Commands,
) {
    if evt.iter().next().is_some() && state.set(ConnectState::Connecting).is_ok() {
        commands.insert_resource(new_client(server_address.0.as_str()));
    }
}

fn in_game_on_connect(
    client: Option<Res<RenetClient>>,
    mut state: ResMut<State<AppState>>
) {
    if let Some(client) = client {
        if client.is_connected() {
            state.set(AppState::InGame).unwrap();
        }
    }
}
