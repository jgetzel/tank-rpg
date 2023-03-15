use bevy::prelude::{Camera, GlobalTransform, KeyCode, MouseButton, Query, Res, ResMut, Window, With};
use bevy::math::Vec2;
use bevy::input::Input;
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;
use crate::client_networking::client_input::ClientInput;
use crate::display::camera::MainCamera;
use crate::utils::ndc::{screen_to_world, ScreenSize};

pub fn keyboard_events(
    keys: Res<Input<KeyCode>>,
    mut input: ResMut<ClientInput>,
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
    window_q: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut input: ResMut<ClientInput>,
) {
    let Ok((camera, camera_transform)) = q_camera.get_single()
        else { return; };

    let Ok(window) = window_q.get_single()
        else { return; };

    let Some(screen_pos) = window.cursor_position() else { return; };

    input.mouse_pos = screen_to_world(screen_pos, window.screen_size(), camera, camera_transform);
}

pub fn mouse_click(
    mut input: ResMut<ClientInput>,
    button: ResMut<Input<MouseButton>>,
) {
    input.fire_bullet = button.just_pressed(MouseButton::Left);
}
