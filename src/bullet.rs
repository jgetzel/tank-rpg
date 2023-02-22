use bevy::app::App;
use bevy::log::debug;
use bevy::prelude::{Children, Commands, Component, Entity, GlobalTransform, info, Plugin, Quat, Query, Transform, TransformBundle, Vec3, With};
use bevy::utils::default;
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::prelude::{RigidBody, Sensor, Velocity};
use crate::assets::SpriteEnum;
use crate::player::components::PlayerInput;
use crate::environment::BULLET_LAYER;
use crate::object::components::Object;
use crate::player::components::{Player, PlayerTurret};

static BULLET_COLLIDER_RADIUS: f32 = 10.;
static BULLET_OFFSET: f32 = 60.;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(fire_bullet);
    }
}

#[derive(Component)]
pub struct Bullet {
    pub owner: Option<Entity>,
}


fn fire_bullet(
    mut commands: Commands,
    query: Query<(Entity, &PlayerInput, &Children), With<Player>>,
    turret_query: Query<(&PlayerTurret, &GlobalTransform)>,
) {
    query.iter().for_each(|(ent, input, children)| {
        if !input.fire_bullet { return; }
        children.iter().for_each(|&child| {
            let Ok((turret, trans)) = turret_query.get(child)
                else { return; };
            let angle = turret.direction.y.atan2(turret.direction.x);
            let start_pos = trans.translation().truncate() + turret.direction * BULLET_OFFSET;
            commands.spawn((
                Bullet {
                    owner: Some(ent),
                },
                SpriteEnum::Bullet,
                TransformBundle::from_transform(Transform {
                    translation: start_pos.extend(BULLET_LAYER),
                    rotation: Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle),
                    ..default()
                }),
                Object::new(),
                Velocity::linear(turret.direction * turret.bullet_speed),
                RigidBody::KinematicVelocityBased,
                Collider::ball(BULLET_COLLIDER_RADIUS),
                Sensor,
            ));
        });
    });
}