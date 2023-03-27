use bevy::prelude::{Bundle, Camera2dBundle, default, Entity, OrthographicProjection, SpatialBundle, Sprite, Transform, TransformBundle};
use bevy::core::Name;
use bevy::sprite::Anchor;
use bevy::math::Vec2;
use bevy_rapier2d::dynamics::{Damping, LockedAxes, RigidBody, Velocity};
use bevy_rapier2d::geometry::Collider;
use crate::asset_loader::components::SpriteEnum;
use crate::display::camera::MainCamera;
use crate::simulation::server_sim::player::{Health, Player, PlayerTurret};
use crate::display::sprite_updater::{AutoSorted, BACKGROUND_LAYER, CAMERA_LAYER, PLAYER_LAYER};
use crate::simulation::server_sim::spawn::SpawnPoint;

const TREE_ANCHOR: [f32; 2] = [0., -0.22];
const TREE_TRUNK_ANCHOR: [f32; 2] = [0., -0.375];
const _TURRET_ANCHOR: [f32; 2] = [-0.045, 0.15];
const TURRET_POSITION: [f32; 2] = [-7., 27.];
const TANK_COLLIDER_RADIUS: f32 = 45.;

pub fn default_camera() -> impl Bundle {
    (
        Camera2dBundle {
            transform: Transform::from_xyz(0., 0., CAMERA_LAYER),
            projection: OrthographicProjection {
                scale: 1.5,
                ..default()
            },
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

pub fn tree() -> impl Bundle {
    (
        Name::new("Tree"),
        AutoSorted,
        SpriteEnum::Tree,
        Sprite {
            anchor: Anchor::Custom(Vec2::from(TREE_ANCHOR)),
            ..default()
        },
        TransformBundle::default(),
        Collider::ball(100.),
    )
}

pub fn tree_trunk() -> impl Bundle {
    (
        Name::new("Tree Trunk"),
        AutoSorted,
        SpriteEnum::TreeTrunk1,
        Sprite {
            anchor: Anchor::Custom(Vec2::from(TREE_TRUNK_ANCHOR)),
            ..default()
        },
        TransformBundle::default(),
        Collider::ball(75.),
    )
}

pub fn tree_leaves() -> impl Bundle {
    (
        Name::new("Tree Leaves"),
        SpriteEnum::TreeLeaves1,
        TransformBundle::from(Transform::from_xyz(0., 217., 0.))
    )
}

pub fn spawn_point(position: Vec2) -> impl Bundle {
    (
        TransformBundle::from_transform(Transform::from_xyz(position.x, position.y, 0.)),
        SpawnPoint
    )
}

pub fn get_player_bundle(id: u64, position: Option<Vec2>) -> impl Bundle {
    let position = match position {
        Some(position) => position,
        None => Vec2::default()
    };

    (
        Name::from(format!("Player {id}")),
        AutoSorted,
        Player::new(id),
        Health::default(),
        SpriteEnum::TankDefault,
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

pub fn get_turret_bundle(owner: Entity) -> impl Bundle {
    (
        Name::from("Turret"),
        PlayerTurret::new(owner),
        SpriteEnum::TankDefaultTurret,
        Sprite {
            anchor: Anchor::Custom(Vec2::from(_TURRET_ANCHOR)),
            ..default()
        },
        Transform {
            translation: Vec2::from(TURRET_POSITION).extend(0.),
            ..default()
        }
    )
}
