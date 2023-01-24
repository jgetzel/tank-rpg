use bevy::math::{Vec2, Vec3};
use bevy::prelude::{BuildChildren, Commands, default, Res, Transform};
use bevy::sprite::{Anchor, Sprite, SpriteBundle};
use bevy_rapier2d::dynamics::Velocity;
use bevy_rapier2d::prelude::{Collider, Damping, LockedAxes, RigidBody};
use crate::game_plugins::assets::GameAssets;
use crate::game_plugins::player::{Player, PlayerTurret};

static BACKGROUND_LAYER: f32 = -10.;
static PLAYER_LAYER: f32 = 0.;
pub static BULLET_LAYER: f32 = 1.;
static TURRET_LAYER: f32 = 2.;
pub static CAMERA_LAYER: f32 = 100.;

static TANK_SCALE: f32 = 2. / 3.;
static TURRET_ANCHOR: [f32; 2] = [-0.18, 0.];
static TURRET_POSITION: [f32; 2] = [0., 30.];
static TANK_COLLIDER_RADIUS: f32 = 60.;

pub fn init_player(position: Vec2) -> impl Fn(Commands, Res<GameAssets>) {
    move |mut commands: Commands, game_assets: Res<GameAssets>| {
        commands.spawn((
            Player::default(),
            SpriteBundle {
                texture: game_assets.tank_gray.clone(),
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
        ))
            .with_children(|parent| {
                parent.spawn((
                    PlayerTurret::default(),
                    SpriteBundle {
                        texture: game_assets.tank_gray_turret.clone(),
                        sprite: Sprite {
                            anchor: Anchor::Custom(Vec2::from(TURRET_ANCHOR)),
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec2::from(TURRET_POSITION).extend(TURRET_LAYER),
                            ..default()
                        },
                        ..default()
                    }
                ));
            });
    }
}

pub fn init_background(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn(SpriteBundle {
        texture: game_assets.background.clone(),
        transform: Transform::from_xyz(0., 0., BACKGROUND_LAYER),
        ..default()
    });
}