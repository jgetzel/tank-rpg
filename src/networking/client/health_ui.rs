use bevy::app::App;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContexts};
use crate::camera::MainCamera;
use crate::player::Health;
use crate::scenes::AppState;
use crate::utils::ndc::world_to_screen;

pub struct ClientHealthDisplayPlugin;

impl Plugin for ClientHealthDisplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(UIWidth(0.))
            .add_system(display_health.in_set(OnUpdate(AppState::InGame)));
    }
}

#[derive(Resource)]
struct UIWidth(pub f32);

fn display_health(
    mut contexts: EguiContexts,
    healths: Query<(Entity, &GlobalTransform, &Health)>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut ui_width: ResMut<UIWidth>,
) {
    let Ok((cam, cam_trans)) = camera_q.get_single() else { return; };
    let Ok(window) = window_q.get_single() else { return; };

    healths.iter().for_each(|(ent, trans, health)| {
        let area = egui::Area::new(format!("{} Health Area", ent.index()))
            .fixed_pos(
                (world_to_screen(
                    trans.translation().truncate(), window.height(),
                    cam, cam_trans) - Vec2::new(ui_width.0/ 2., 75.))
                    .as_ref()
            )
            .interactable(false)
            .show(contexts.ctx_mut(), |ui| {
                egui::Frame::dark_canvas(&egui::Style::default())
                    .show(ui, |ui| {
                        ui.label(format!("{}/{}", health.health, health.max_health));
                    });
            });

        ui_width.0 = area.response.rect.width();
    });
}