use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::utils::HashMap;
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::{ComboBox, DragValue, Slider};
use bevy_rapier2d::prelude::{DebugRenderContext, RapierDebugRenderPlugin};
use once_cell::sync::Lazy;
use crate::AppState;
use crate::client_ui::client_debug::ActiveWindowEnum::*;
use crate::simulation::SyncedObjects;
use crate::simulation::Lobby;

pub struct ClientDebugUIPlugin;

impl Plugin for ClientDebugUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DebugToggle::default())
            .insert_resource(ActiveWindow::default())
            .add_system(debug_toggle_sys)
            .add_system(choose_window.run_if(is_debug_on))
            .add_system(sprite_anchor_edit.run_if(is_window_active(SpriteAnchorEdit)))
            .add_system(transform_edit.run_if(is_window_active(TransformEdit)))
            .add_system(show_player_lobby
                .in_set(OnUpdate(AppState::InGame))
                .run_if(is_window_active(PlayerLobby))
            );


        app.add_plugin(RapierDebugRenderPlugin::default());
            // .insert_resource(DebugRenderContext {
            //     enabled: true, ..default()
            // })
            // .add_system(toggle_debug_render);
    }
}

#[derive(Resource, Default)]
pub struct DebugToggle {
    debug_mode: bool,
}

#[derive(Resource, Default)]
pub struct ActiveWindow(pub ActiveWindowEnum);

#[derive(Default, PartialEq, Eq, Hash)]
pub enum ActiveWindowEnum {
    #[default]
    ChooseWindow,
    PlayerLobby,
    SpriteAnchorEdit,
    TransformEdit,
}

static ACTIVE_WINDOW_NAME_MAP: Lazy<HashMap<ActiveWindowEnum, &str>> = Lazy::new(|| {
    HashMap::from([
        (ChooseWindow, "Choose Window"),
        (PlayerLobby, "Player Lobby"),
        (SpriteAnchorEdit, "Sprite Anchor Edit"),
        (TransformEdit, "Transform Edit")
    ])
});

pub fn is_debug_on(
    debug_toggle: Res<DebugToggle>,
) -> bool {
    debug_toggle.debug_mode
}

pub fn is_window_active(window: ActiveWindowEnum) -> impl FnMut(Res<ActiveWindow>, Res<DebugToggle>) -> bool {
    move |active_window: Res<ActiveWindow>, debug: Res<DebugToggle>| {
        if debug.debug_mode {
            active_window.0 == window
        } else { false }
    }
}

fn debug_toggle_sys(
    inputs: Res<Input<ScanCode>>,
    mut debug_toggle: ResMut<DebugToggle>,
) {
    if inputs.just_pressed(ScanCode(0x29)) {
        debug_toggle.debug_mode = !debug_toggle.debug_mode;
    };
}

fn toggle_debug_render(
    debug: Res<DebugToggle>,
    rapier_debug_config: Option<ResMut<DebugRenderContext>>
) {
    let Some(mut config) = rapier_debug_config else { return; };
    config.enabled = debug.debug_mode;
}

fn choose_window(
    mut contexts: EguiContexts,
    mut active_window: ResMut<ActiveWindow>,
) {
    egui::Window::new("Choose Debug Window")
        .show(contexts.ctx_mut(), |ui| {
            ComboBox::from_label("Choose Window")
                .selected_text(*ACTIVE_WINDOW_NAME_MAP.get(&active_window.0).unwrap())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut active_window.0, ChooseWindow, "Choose Window");
                    ui.selectable_value(&mut active_window.0, PlayerLobby, "Player Lobby");
                    ui.selectable_value(&mut active_window.0, SpriteAnchorEdit, "Sprite Anchor Format");
                    ui.selectable_value(&mut active_window.0, TransformEdit, "Transform Edit")
                });
        });
}

fn show_player_lobby(
    mut egui_ctx: EguiContexts,
    debug_mode: Res<DebugToggle>,
    lobby: Res<Lobby>,
    objects: Res<SyncedObjects>,
) {
    if !debug_mode.debug_mode { return; }

    egui::Window::new("Client Info")
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.label("Lobby");
            ui.group(|ui| {
                lobby.player_data.iter().for_each(|player| {
                    ui.label(format!("Player {}: Entity {:?}", player.0, player.1.clone()));
                });
            });

            ui.separator();

            ui.label("Objects");
            ui.group(|ui| {
                objects.objects.iter().for_each(|(object_id, ent)| {
                    ui.label(format!("ObjectID {}: Entity {:?}", object_id, ent));
                });
            });
        });
}

fn sprite_anchor_edit(
    mut query: Query<(&mut Sprite, &Name)>,
    mut contexts: EguiContexts,
) {
    egui::Window::new("Sprite Anchor Edit")
        .show(contexts.ctx_mut(), |ui| {
            query.iter_mut().for_each(|(mut sprite, name)| {
                let Anchor::Custom(anchor) = &mut sprite.anchor else { return; };

                ui.columns(3, |columns| {
                    columns[0].label(name.to_string());
                    columns[1].add(Slider::new(&mut anchor.x, -0.5..=0.5));
                    columns[2].add(Slider::new(&mut anchor.y, -0.5..=0.5));
                });
                ui.separator();
            });
        });
}

fn transform_edit(
    mut query: Query<(&mut Transform, &Name)>,
    mut contexts: EguiContexts,
) {
    egui::Window::new("Transform Edit")
        .show(contexts.ctx_mut(), |ui| {
            query.iter_mut().for_each(|(mut trans, name)| {
                ui.columns(3, |columns| {
                    columns[0].label(name.to_string());
                    columns[1].add(DragValue::new(&mut trans.translation.x));
                    columns[2].add(DragValue::new(&mut trans.translation.y));
                });
                ui.separator();
            });
        });
}