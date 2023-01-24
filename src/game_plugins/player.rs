use bevy::math::{Quat, Vec2, Vec3};
use bevy::prelude::{
    BuildChildren, Commands, Component, default, Entity, GlobalTransform, Query, Res,
    Sprite, SpriteBundle, Time, Transform,
};
use bevy::sprite::Anchor;
use bevy_rapier2d::dynamics::Velocity;
use bevy_rapier2d::prelude::{Collider, Damping, LockedAxes, RigidBody};
use crate::game_plugins::assets::GameAssets;
use crate::game_plugins::environment::{PLAYER_LAYER, TURRET_LAYER};
use crate::game_plugins::input_helper::Input;

#[derive(Component)]
pub struct Player {
    pub accel: f32,
    pub max_speed: f32,
    pub friction: f32,
    pub curr_velocity: Vec2,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            accel: 2400.,
            max_speed: 300.,
            friction: 500.,
            curr_velocity: Vec2::default(),
        }
    }
}

#[derive(Component)]
pub struct PlayerTurret {
    pub owner: Option<Entity>,
    pub direction: Vec2,
    pub bullet_speed: f32,
}

impl Default for PlayerTurret {
    fn default() -> Self {
        PlayerTurret {
            owner: None,
            direction: Vec2::default(),
            bullet_speed: 600.,
        }
    }
}

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

pub fn player_move(
    input: Res<Input>,
    mut query: Query<(&mut Velocity, &Player)>,
    time: Res<Time>,
) {
    for (mut velocity, player) in query.iter_mut() {
        let new_velocity = velocity.linvel + (player.accel * input.movement * time.delta_seconds());
        velocity.linvel =
            if [player.max_speed, velocity.linvel.length()].iter().all(|v| new_velocity.length() > *v) {
                new_velocity.clamp_length_max(player.max_speed)
            } else {
                new_velocity
            };
    }
}

pub fn player_turret_rotate(
    input: Res<Input>,
    mut query: Query<(&mut Transform, &GlobalTransform, &mut PlayerTurret)>,
) {
    for (mut trans, global_trans, mut turret) in query.iter_mut() {
        let diff = input.mouse_position - global_trans.translation().truncate();
        let angle = diff.y.atan2(diff.x);
        trans.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);

        turret.direction = diff.normalize();
    }
}
