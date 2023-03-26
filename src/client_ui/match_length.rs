use bevy::app::App;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::{Align2, RichText};
use crate::AppState;
use crate::simulation::server_sim::match_ffa::MatchTimer;
use crate::utils::math;
use crate::utils::ui::DEFAULT_FRAME;

pub struct MatchLengthUIPlugin;

impl Plugin for MatchLengthUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(match_length_ui.in_set(OnUpdate(AppState::InGame)));
    }
}

pub const WIDTH: f32 = 100.;

fn match_length_ui(
    mut contexts: EguiContexts,
    match_timer: Option<Res<MatchTimer>>,
) {
    let Some(match_timer) = match_timer else { return; };

    let time_remaining_int = match_timer.time_remaining.floor() as u32;

    let time_remaining_string = math::seconds_to_formatted_time_string(time_remaining_int);

    egui::Area::new("Match Timer Area")
        .anchor(Align2::RIGHT_TOP, [0., 0.])
        .show(contexts.ctx_mut(), |ui| {
            DEFAULT_FRAME.outer_margin(10.0).show(ui, |ui| {
                ui.set_width(WIDTH);
                ui.vertical_centered( |ui| {
                    ui.label(RichText::new(time_remaining_string).heading());
                });
            });
        });
}
