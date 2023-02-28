use bevy::prelude::{Bundle, Camera2dBundle, default, Transform};
use crate::asset_loader::components::SpriteEnum;
use crate::camera::MainCamera;
use crate::sprite_updater::{BACKGROUND_LAYER, CAMERA_LAYER};

pub fn default_camera() -> impl Bundle {
    (
        Camera2dBundle {
            transform: Transform::from_xyz(0., 0., CAMERA_LAYER),
            ..default()
        },
        MainCamera
    )
}

pub fn default_background() -> impl Bundle {
    (
        SpriteEnum::Background,
        Transform::from_xyz(0., 0., BACKGROUND_LAYER)
    )
}