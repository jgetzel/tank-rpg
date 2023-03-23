use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::{Align2, Color32, Pos2, Shape};
use bevy_egui::egui::epaint::CircleShape;
use crate::AppState;
use crate::display::camera::MainCamera;
use crate::simulation::server_sim::player::{Player, You};
use crate::utils::ndc;
use crate::utils::ui::DEFAULT_FRAME;

pub struct MiniMapUIPlugin;

impl Plugin for MiniMapUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(minimap_display.in_set(OnUpdate(AppState::InGame)));
    }
}

const SIZE: f32 = 200.;

fn minimap_display(
    mut contexts: EguiContexts,
    player_q: Query<&Transform, With<Player>>,
    you_q: Query<&Transform, With<You>>,
    window: Query<&Window, With<PrimaryWindow>>,
    cam: Query<(&Camera, &GlobalTransform), With<MainCamera>>
) {
    let response = egui::Area::new("Minimap Area")
        .anchor(Align2::RIGHT_BOTTOM, [0., 0.])
        .show(contexts.ctx_mut(), |ui| {
            DEFAULT_FRAME.outer_margin(10.0).show(ui, |ui| {
                ui.set_width(SIZE);
                ui.set_height(SIZE);
            });
        }).response;


    let painter = egui::Painter::new(
        contexts.ctx_mut().clone(),
        response.layer_id,
        response.rect);

    let (cam, cam_trans) = cam.get_single().unwrap();
    let window = window.get_single().unwrap();
    let world_to_screen = |trans: Vec2| {
        ndc::world_to_screen(trans,
                             window.height(),
                             cam,cam_trans)
    };

    let min = Vec2::new(response.rect.min.x, response.rect.min.y);
    let max = Vec2::new(response.rect.max.x, response.rect.max.y);

    let minimap_center = (min + max) / 2.;

    let you_trans = you_q.get_single().unwrap_or(&Transform::default()).translation.truncate();
    let you_screen = world_to_screen(you_trans);
    let you_minimap_diff = minimap_center - you_screen;

    player_q.iter().for_each(|trans| {
        let trans = trans.translation.truncate();
        let screen_pos = world_to_screen(trans) + you_minimap_diff;
        let color = if trans == you_trans { Color32::WHITE } else { Color32::RED };

        painter.add(Shape::Circle(CircleShape::filled(
            Pos2::from([screen_pos.x, screen_pos.y]),
            5., color
        )));
    });
}