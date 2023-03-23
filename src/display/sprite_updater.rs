use bevy::app::App;
use bevy::asset::{Assets, Handle};
use bevy::log::info;
use bevy::math::{IVec2, UVec2, Vec2};
use bevy::prelude::{Camera, Commands, Component, default, Entity, GlobalTransform, Image, IntoSystemConfig, OnUpdate, OrthographicProjection, Plugin, Query, Res, ResMut, Resource, Sprite, SpriteBundle, Transform, TransformBundle, Window, With};
use bevy::reflect::List;
use bevy::utils::{HashMap, HashSet};
use bevy::window::PrimaryWindow;
use bevy_egui::egui::emath;
use crate::AppState;
use crate::asset_loader::components::SpriteEnum;
use crate::asset_loader::resources::SpriteAssets;
use crate::display::camera::MainCamera;
use crate::utils::ndc::{camera_world_bounds, ScreenSize};

pub const BACKGROUND_LAYER: f32 = -100.;
pub const PLAYER_LAYER: f32 = 0.;
pub const BULLET_LAYER: f32 = 1.;
pub const CAMERA_LAYER: f32 = 100.;

pub struct SpriteUpdatePlugin;

impl Plugin for SpriteUpdatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BackgroundTilePositions::default());

        app.add_systems((
            update_sprite_handle,
            auto_sort_system,
            background_spawner.in_set(OnUpdate(AppState::InGame)),
        ));
    }
}

#[derive(Component)]
pub struct AutoSorted;

#[allow(clippy::type_complexity)]
fn update_sprite_handle(
    q: Query<(Entity, &SpriteEnum, Option<&Handle<Image>>, Option<&Sprite>, Option<&Transform>)>,
    assets: Res<SpriteAssets>,
    mut commands: Commands,
) {
    q.iter().for_each(|(ent, &sprite_enum, handle, sprite_info, trans, )| {
        enum ActionType {
            UpdateHandle,
            InsertBundle,
            Neither,
        }

        let action = match handle {
            Some(handle) => {
                if handle != &assets.get(sprite_enum)
                { ActionType::UpdateHandle } else { ActionType::Neither }
            }
            None => ActionType::InsertBundle,
        };
        match action {
            ActionType::UpdateHandle => { commands.entity(ent).insert(assets.get(sprite_enum)); }
            ActionType::InsertBundle => {
                let mut ent_commands = commands.entity(ent);
                let ent_commands = ent_commands.insert(
                    SpriteBundle {
                        texture: assets.get(sprite_enum),
                        ..default()
                    });

                if let Some(sprite_info) = sprite_info {
                    ent_commands.insert(sprite_info.clone());
                }
                if let Some(&trans) = trans {
                    ent_commands.insert(trans);
                }
            }
            ActionType::Neither => {}
        };
    });
}

fn auto_sort_system(
    mut q: Query<&mut Transform, With<AutoSorted>>
) {
    q.iter_mut().for_each(|mut trans| {
        trans.translation.z = -trans.translation.y / 10000.;
    });
}

#[derive(Resource, Default)]
pub struct BackgroundTilePositions {
    pub set: HashSet<IVec2>,
}

fn background_spawner(
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    window: Query<&Window, With<PrimaryWindow>>,
    assets: Res<SpriteAssets>,
    image_assets: Res<Assets<Image>>,
    mut tile_pos_set: ResMut<BackgroundTilePositions>,
    mut commands: Commands,
) {
    let Ok((cam, trans)) = camera.get_single() else { return; };
    let Ok(window) = window.get_single() else { return; };

    let [min, max] =
        camera_world_bounds(window.screen_size(), cam, trans);

    let handle = assets.get(SpriteEnum::Background);
    let Some(image) = image_assets.get(&handle) else { return; };
    let background_size = image.size();

    fn generate_sprite_positions(
        camera_bounds_min: Vec2,
        camera_bounds_max: Vec2,
        sprite_size: Vec2,
        padding: f32,
    ) -> Vec<Vec2> {
        let mut positions = Vec::new();

        let padded_bounds_min = camera_bounds_min - Vec2::splat(padding);
        let padded_bounds_max = camera_bounds_max + Vec2::splat(padding);

        let start_col = (padded_bounds_min.x / sprite_size.x).floor() as i32;
        let start_row = (padded_bounds_min.y / sprite_size.y).floor() as i32;
        let end_col = (padded_bounds_max.x / sprite_size.x).ceil() as i32;
        let end_row = (padded_bounds_max.y / sprite_size.y).ceil() as i32;

        for row in start_row..end_row {
            for col in start_col..end_col {
                let x = col as f32 * sprite_size.x;
                let y = row as f32 * sprite_size.y;
                let position = Vec2::new(x, y);
                positions.push(position);
            }
        }
        positions
    }

    let points_within_camera: Vec<Vec2> = generate_sprite_positions(
        min,
        max,
        background_size,
        background_size.x,
    );

    points_within_camera.into_iter().filter(|p| tile_pos_set.set.insert(p.as_ivec2()))
        .for_each(|p| {
            commands.spawn(SpriteEnum::Background)
                .insert(TransformBundle::from_transform(
                    Transform::from_xyz(p.x, p.y, BACKGROUND_LAYER))
                );
        });
}
