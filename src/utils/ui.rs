use bevy::utils::default;
use bevy_egui::egui::{Color32, Frame, Margin, Rounding, Visuals};
use bevy_egui::egui::epaint::Shadow;
use once_cell::sync::Lazy;

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