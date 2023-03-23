use std::ops::Add;
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::{Align2, Color32, emath, Pos2, Shape, Stroke};
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
    const OUTER_MARGIN: f32 = 10.;
    const MINIMAP_SCALE: f32 = 0.1;
    let response = egui::Area::new("Minimap Area")
        .anchor(Align2::RIGHT_BOTTOM, [0., 0.])
        .show(contexts.ctx_mut(), |ui| {
            DEFAULT_FRAME.outer_margin(OUTER_MARGIN).show(ui, |ui| {
                ui.set_width(SIZE);
                ui.set_height(SIZE);
            });
        }).response;

    let Ok(you_trans) = you_q.get_single() else { return; };
    let you_trans = you_trans.translation.truncate();

    let (cam, cam_trans) = cam.get_single().unwrap();
    let window = window.get_single().unwrap();
    let world_to_screen = |trans: Vec2| {
        ndc::world_to_screen(trans,
                             window.height(),
                             cam,cam_trans)
    };


    let min = Vec2::new(response.rect.min.x + OUTER_MARGIN, response.rect.min.y + OUTER_MARGIN);
    let max = Vec2::new(response.rect.max.x - OUTER_MARGIN, response.rect.max.y - OUTER_MARGIN);

    let minimap_center = (min + max) / 2.;

    let you_screen = world_to_screen(you_trans);

    let painter = egui::Painter::new(
        contexts.ctx_mut().clone(),
        response.layer_id,
        response.rect);

    player_q.iter().for_each(|trans| {
        let trans = trans.translation.truncate();
        let relative_screen_pos = you_screen - world_to_screen(trans);
        let scaled_relative_pos = relative_screen_pos * MINIMAP_SCALE;
        let screen_pos = (minimap_center - scaled_relative_pos).clamp(min, max);

        let color = if trans == you_trans { Color32::WHITE } else { Color32::RED };

        let outside_bounds = [min, max].into_iter()
            .any(|v| screen_pos.x == v.x || screen_pos.y == v.y);

        let screen_pos2 = Pos2::from([screen_pos.x, screen_pos.y]);
        let shape = if !outside_bounds {
            Shape::Circle(CircleShape::filled(screen_pos2, 5., color))
        }
        else {
            let relative_norm = relative_screen_pos.normalize() * 5.;
            let vec = emath::vec2(relative_norm.x, relative_norm.y);
            Shape::LineSegment {
                points: [screen_pos2.add(vec), screen_pos2],
                stroke: Stroke { width: 3., color},
            }
        };

        painter.add(shape);
    });
}