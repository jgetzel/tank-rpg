use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Bundle, default, SpatialBundle, Transform};
use bevy_rapier2d::dynamics::{Damping, LockedAxes, RigidBody, Velocity};
use bevy_rapier2d::geometry::Collider;
use crate::assets::SpriteEnum;
use crate::environment::{PLAYER_LAYER, TURRET_LAYER};
use crate::input_helper::PlayerInput;
use crate::object::Object;
use crate::player::{Player, PlayerTurret};

const TANK_SCALE: f32 = 2. / 3.;
const _TURRET_ANCHOR: [f32; 2] = [-0.18, 0.];
const TURRET_POSITION: [f32; 2] = [0., 30.];
const TANK_COLLIDER_RADIUS: f32 = 60.;

pub fn get_player_bundle(id: u64, position: Option<Vec2>) -> impl Bundle {
    let position = match position {
        Some(position) => position,
        None => Vec2::default()
    };

    (
        Player::new(id),
        PlayerInput::default(),
        Object::new(),
        SpriteEnum::TankGray,
        SpatialBundle {
            transform: Transform {
                scale: Vec3::ONE * TANK_SCALE,
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
        PlayerTurret::default(),
        Transform {
            translation: Vec2::from(TURRET_POSITION).extend(TURRET_LAYER),
            ..default()
        }
    )
}
