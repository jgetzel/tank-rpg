use bevy::prelude::default;
use bevy_egui::egui::*;
use bevy_egui::egui::epaint::Shadow;
use once_cell::sync::Lazy;

pub const WIDTH: f32 = 100.;

pub static DEFAULT_FRAME: Lazy<Frame> = Lazy::new(||
    Frame {
        inner_margin: Margin::from(10.),
        rounding: Rounding::from(5.0),
        shadow: Shadow {
            extrusion: 4.,
            color: Color32::from_black_alpha(50),
        },
        fill: Visuals::default().window_fill(),
        ..default()
    }
);