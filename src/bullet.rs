use bevy::prelude::{Children, Commands, Component, Entity, GlobalTransform, Query, With};
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::prelude::{RigidBody, Sensor, Velocity};
use crate::input_helper::PlayerInput;
use crate::object::components::Object;
use crate::player::components::{Player, PlayerTurret};

static BULLET_COLLIDER_RADIUS: f32 = 10.;
static BULLET_OFFSET: f32 = 60.;

#[derive(Component)]
pub struct Bullet {
    pub owner: Option<Entity>,
}


pub fn fire_bullet(
    mut commands: Commands,
    query: Query<(Entity, &PlayerInput, &Children), With<Player>>,
    turret_query: Query<(&PlayerTurret, &GlobalTransform)>,
    // assets: Res<GameAssets>,
) {

    for (entity, input, children) in query.iter() {
        if !input.fire_bullet { return; };
        for child in children.iter() {
            let Ok((turret, trans)) = turret_query.get(*child)
                else { continue; };

            // let angle = turret.direction.y.atan2(turret.direction.x);
            // let start_pos = trans.translation().truncate() + turret.direction * BULLET_OFFSET;
            let bullet = commands.spawn((
                Bullet {
                    owner: Some(entity),
                },
                // SpriteBundle {
                //     texture: assets.bullet.clone(),
                //     transform: Transform {
                //         translation: start_pos.extend(BULLET_LAYER),
                //         rotation: Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle),
                //         ..default()
                //     },
                //     ..default()
                // },
            )).id();

            commands.entity(bullet)
                .insert((
                    Object::new(),
                    RigidBody::KinematicVelocityBased,
                    Collider::ball(BULLET_COLLIDER_RADIUS),
                    Sensor,
                    Velocity::linear(turret.direction * turret.bullet_speed),
                ));
        }
    }
}