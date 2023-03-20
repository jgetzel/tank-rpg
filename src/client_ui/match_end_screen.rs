use bevy::app::App;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::*;
use crate::AppState;
use crate::simulation::{Lobby, PlayerData};
use crate::simulation::server_sim::match_ffa::is_match_finished;
use crate::utils::networking::messages::PlayerId;

pub struct MatchEndScreenUIPlugin;

impl Plugin for MatchEndScreenUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(
                match_end_screen
                    .run_if(is_match_finished)
                    .in_set(OnUpdate(AppState::InGame))
            );
    }
}

fn match_end_screen(
    mut contexts: EguiContexts,
    lobby: Res<Lobby>,
) {
    egui::Area::new("Final Leaderboard Area")
        .anchor(Align2::CENTER_CENTER, [0., 0.])
        .show(contexts.ctx_mut(), |ui| {
            ui.set_width_range(500.0..=1000.0);
            egui::Frame::menu(&egui::Style::default())
                .outer_margin(10.0)
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.set_height(5.);
                        ui.label("Leaderboard");
                    });
                    ui.separator();
                    egui::Frame::default()
                        .outer_margin(5.0)
                        .show(ui, |ui| {
                            ui.columns(6, |columns| {
                                columns[0].label("Place");
                                columns[1].label("Name");
                                columns[4].label("Kills");
                                columns[5].label("Deaths");
                            });
                        });
                    egui::Frame::group(&egui::Style::default()).show(ui, |ui| {
                        ui.columns(6, |columns| {
                            let mut player_vec = lobby.player_data.clone().into_iter()
                                .collect::<Vec<(PlayerId, PlayerData)>>();

                            player_vec.sort_by(|(_, data_a), (_, data_b)| {
                                data_b.kills.cmp(&data_a.kills)
                            });

                            player_vec.iter().enumerate().for_each(|(index, (id, data))| {
                                columns[0].label(format!("#{}", index + 1));
                                columns[1].label(format!("Player {}", id));
                                columns[4].label(format!("{}", data.kills));
                                columns[5].label(format!("{}", data.deaths));
                            });
                        });
                    });
                });
        });
}