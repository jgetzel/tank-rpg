use bevy::prelude::*;
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::prelude::{ActiveEvents, CollisionEvent, RigidBody, Sensor, Velocity};
use crate::asset_loader::components::SpriteEnum;
use crate::bullet::BulletSystemStage::{CollisionHandle, CollisionSend};
use crate::player::components::PlayerInput;
use crate::sprite_updater::BULLET_LAYER;
use crate::object::components::Object;
use crate::player::components::{Player, PlayerTurret};
use crate::player::{DeathEvent, Health};
use crate::utils::CustomDespawn;

static BULLET_COLLIDER_RADIUS: f32 = 10.;
static BULLET_OFFSET: f32 = 60.;
static BULLET_LIFETIME: f32 = 3.0;
static BULLET_DAMAGE: f32 = 5.0;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BulletCollisionEvent>()
            .add_system(fire_bullet)
            .add_system(bullet_decay)
            .add_system(bullet_collision_sender.in_set(CollisionSend))
            .add_system(bullet_collision_handler.in_set(CollisionHandle)
                .after(CollisionSend));
    }
}

#[derive(Component)]
pub struct Bullet {
    pub owner: Option<Entity>,
    pub lifetime: f32,
    pub damage: f32,
}

#[derive(Debug)]
pub struct BulletCollisionEvent {
    pub bullet: Entity,
    pub player: Entity
}

#[derive(SystemSet, Debug, Eq, PartialEq, Hash, Clone)]
pub enum BulletSystemStage {
    CollisionSend,
    CollisionHandle
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
                Name::from("Bullet"),
                Bullet {
                    owner: Some(ent),
                    lifetime: BULLET_LIFETIME,
                     damage: BULLET_DAMAGE,
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
                ActiveEvents::COLLISION_EVENTS,
            ));
        });
    });
}

fn bullet_decay(
    mut bullets: Query<(Entity, &mut Bullet)>,
    time: Res<Time>,
    mut commands: Commands
) {
    bullets.iter_mut().for_each(|(ent, mut bullet)| {
        bullet.lifetime -= time.delta_seconds();
        if bullet.lifetime <= 0. {
            commands.entity(ent).custom_despawn();
        }
    })
}

fn bullet_collision_sender(
    mut collision_events: EventReader<CollisionEvent>,
    mut bullet_event_wr: EventWriter<BulletCollisionEvent>,
    bullets: Query<(Entity, &Bullet)>,
    players: Query<Entity, With<Player>>
) {
    collision_events.iter().for_each(|&e| {
        if let CollisionEvent::Started(ent1, ent2, _) = e {
            info!("CollisionEvent occurred between {:?} and {:?}", ent1, ent2);
            let pair = [ent1, ent2];
            let bullet: Option<(Entity, &Bullet)> = pair.into_iter().filter_map(|e| {
                bullets.get(e).ok()
            }).next();

            if let Some((bullet_ent, bullet)) = bullet {
                let player_opt = pair.into_iter().find(|&e| {
                   players.get(e).is_ok()
                });

                if let Some(player) = player_opt {
                    if bullet.owner.ne(&player_opt) {
                        bullet_event_wr.send(BulletCollisionEvent { bullet: bullet_ent, player });
                    }
                }
            }
        };
    });
}

fn bullet_collision_handler(
    mut events: EventReader<BulletCollisionEvent>,
    mut death_writer: EventWriter<DeathEvent>,
    bullets: Query<&Bullet>,
    mut healths: Query<(Entity, &mut Health)>
) {
    events.iter().for_each(|e| {
        let Ok((entity, mut health)) = healths.get_mut(e.player) else { return; };
        let Ok(bullet) = bullets.get(e.bullet) else { return; };
        health.health -= bullet.damage;
        if health.health <= 0. {
            death_writer.send(DeathEvent { entity });
        }
        info!("{e:?}");
    })
}