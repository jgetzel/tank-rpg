use bevy::math::Vec2;
use bevy::prelude::{Camera, Component, GlobalTransform, KeyCode, Query, Res, ResMut, Transform, Windows, With};
use bevy::utils::HashMap;
use bevy::ecs::system::Resource;
use bevy::reflect::Reflect;
use bevy::render::camera::RenderTarget;
use crate::camera::MainCamera;
use serde::{Deserialize, Serialize};
use crate::player::You;

#[derive(Default, Serialize, Deserialize, Component, Resource, Reflect, bevy::reflect::FromReflect, Debug)]
pub struct PlayerInput {
    pub movement: Vec2,
    pub turret_dir: Vec2,
}

pub fn keyboard_events(
    keys: Res<bevy::input::Input<KeyCode>>,
    mut input: ResMut<PlayerInput>,
) {
    let key_to_input_map: HashMap<KeyCode, [f32; 2]> = HashMap::from([
        (KeyCode::W, [0., 1.]),
        (KeyCode::A, [-1., 0.]),
        (KeyCode::S, [0., -1.]),
        (KeyCode::D, [1., 0.])
    ]);

    fn keys_to_vec<'a>(
        iter: impl ExactSizeIterator<Item=&'a KeyCode> + Sized,
        key_map: &HashMap<KeyCode, [f32; 2]>) -> Option<Vec2> {
        iter.filter_map(|key_code: &KeyCode| {
            if let Some(&value) = key_map.get(key_code) {
                Some(Vec2::from_array(value))
            } else { None }
        }).reduce(|acc, curr| acc + curr)
    }

    let pressed_update = keys_to_vec(keys.get_just_pressed(), &key_to_input_map);
    let released_update = keys_to_vec(keys.get_just_released(), &key_to_input_map)
        .map(|vec| -vec);

    let input_update = if let Some(pressed) = pressed_update {
        if let Some(released) = released_update {
            Some(pressed + released)
        } else { Some(pressed) }
    } else { released_update };

    if let Some(input_update) = input_update {
        input.movement += input_update;
    }
}

pub fn mouse_position(
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    q_you: Query<&GlobalTransform, With<You>>,
    mut input: ResMut<PlayerInput>,
) {
    let Ok((camera, camera_transform)) = q_camera.get_single()
        else { return; };

    let Ok(you_trans) = q_you.get_single()
        else { return; };

    let Some(window) = (if let RenderTarget::Window(id) = camera.target
    { windows.get(id) } else { windows.get_primary() })
        else { return; };



    let Some(screen_pos) = window.cursor_position() else { return; };
    let window_size = Vec2::new(window.width(), window.height());

    if window_size.x == 0. || window_size.y == 0. { return; }

    // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
    let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

    // matrix for undoing the projection and camera transform
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

    // use it to convert ndc to world-space coordinates
    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

    input.turret_dir = (world_pos.truncate() - you_trans.translation().truncate()).normalize();
}
