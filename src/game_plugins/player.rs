use bevy::math::{Quat, Vec2, Vec3};
use bevy::prelude::{Component, Entity, GlobalTransform, Query, Res, Time, Transform, With};
use bevy_rapier2d::dynamics::Velocity;
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
    pub bullet_speed: f32
}

impl Default for PlayerTurret {
    fn default() -> Self {
        PlayerTurret {
            owner: None,
            direction: Vec2::default(),
            bullet_speed: 600.
        }
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
