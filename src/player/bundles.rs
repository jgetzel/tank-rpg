use bevy::core::Name;
use bevy::math::Vec2;
use bevy::prelude::{Bundle, default, SpatialBundle, Transform};
use bevy::sprite::{Anchor, Sprite};
use bevy_rapier2d::dynamics::{Damping, LockedAxes, RigidBody, Velocity};
use bevy_rapier2d::geometry::Collider;
use crate::asset_loader::components::SpriteEnum;
use crate::object::components::Object;
use crate::player::components::{Player, PlayerTurret};
use crate::sprite_updater::{PLAYER_LAYER, TURRET_LAYER};

const _TURRET_ANCHOR: [f32; 2] = [-0.18, 0.];
const TURRET_POSITION: [f32; 2] = [0., 20.];
const TANK_COLLIDER_RADIUS: f32 = 45.;

pub fn get_player_bundle(id: u64, position: Option<Vec2>) -> impl Bundle {
    let position = match position {
        Some(position) => position,
        None => Vec2::default()
    };

    (
        Name::from(format!("Player {id}")),
        Player::new(id),
        Object::new(),
        SpriteEnum::TankGray,
        SpatialBundle {
            transform: Transform {
                translation: position.extend(PLAYER_LAYER),
                ..default()
            },
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(TANK_COLLIDER_RADIUS),
        LockedAxes::ROTATION_LOCKED,
        Velocity::default(),
        Damping {
            linear_damping: 5.,
            ..default()
        }
    )
}

pub fn get_turret_bundle() -> impl Bundle {
    (
        Name::from("Turret"),
        PlayerTurret::default(),
        SpriteEnum::TankGrayTurret,
        Sprite {
            anchor: Anchor::Custom(Vec2::from(_TURRET_ANCHOR)),
            ..default()
        },
        Transform {
            translation: Vec2::from(TURRET_POSITION).extend(TURRET_LAYER),
            ..default()
        }
    )
}
