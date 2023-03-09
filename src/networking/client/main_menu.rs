use std::net::{SocketAddr};
use std::str::FromStr;
use bevy::app::App;
use bevy::prelude::*;
use bevy::utils::default;
use bevy_egui::egui::{Align2, Color32, Margin};
use bevy_egui::egui::epaint::Shadow;
use bevy_egui::{egui, EguiContexts};
use bevy_quinnet::client::certificate::CertificateVerificationMode;
use bevy_quinnet::client::Client;
use bevy_quinnet::client::connection::{ConnectionConfiguration, ConnectionEvent};
use crate::prefabs::default_camera;
use crate::scenes::{AppState, despawn_all_entities};

#[derive(States, Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ConnectState {
    #[default]
    NotConnected,
    Connecting,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ServerIPInput("".into()))
            .add_state::<ConnectState>()
            .add_event::<AttemptConnectEvent>()
            .add_system(init.in_schedule(OnEnter(AppState::MainMenu)))
            .add_systems(
                (
                    main_menu_gui.run_if(in_state(ConnectState::NotConnected)),
                    connecting_gui.run_if(in_state(ConnectState::Connecting)),
                    connect_attempt_event_listener,
                    in_game_on_connect
                ).in_set(OnUpdate(AppState::MainMenu))
            )
            .add_system(despawn_all_entities.in_schedule(OnExit(AppState::MainMenu)));
    }
}

#[derive(Resource)]
struct ServerIPInput(pub String);

struct AttemptConnectEvent;

fn init(mut commands: Commands) {
    commands.spawn(default_camera());
}

fn main_menu_gui(
    mut egui_ctx: EguiContexts,
    mut server_ip_string: ResMut<ServerIPInput>,
    mut connect_writer: EventWriter<AttemptConnectEvent>,
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
        .anchor(Align2::CENTER_CENTER, [0., 0.])
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

fn connecting_gui(
    mut egui_ctx: EguiContexts,
) {
    egui::Window::new("Connecting...")
        .anchor(Align2::CENTER_CENTER, [0., 0.])
        .collapsible(false)
        .show(egui_ctx.ctx_mut(), |_| {});
}

fn connect_attempt_event_listener(
    mut events: EventReader<AttemptConnectEvent>,
    server_address: Res<ServerIPInput>,
    mut next_state: ResMut<NextState<ConnectState>>,
    mut client: ResMut<Client>,
) {
    if events.iter().next().is_some() {
        info!("Connection attempt tried!");
        next_state.set(ConnectState::Connecting);
        client.open_connection(
            ConnectionConfiguration::from_addrs(
                SocketAddr::from_str(server_address.0.as_str()).unwrap(),
                SocketAddr::from_str("0.0.0.0:0").unwrap()
            ),
            CertificateVerificationMode::SkipVerification
        ).unwrap();
    }
}

fn in_game_on_connect(
    mut connect_event: EventReader<ConnectionEvent>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if connect_event.iter().next().is_some() {
        next_state.set(AppState::InGame);
    }
}
