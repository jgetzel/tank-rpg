use bevy::prelude::*;
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::prelude::{ActiveEvents, CollisionEvent, RigidBody, Sensor, Velocity};
use crate::asset_loader::components::SpriteEnum;
use crate::simulation::Lobby;
use crate::simulation::server_sim::player::components::PlayerInput;
use crate::display::sprite_updater::{AutoSorted, BULLET_LAYER};
use crate::ServerSet::ServerUpdate;
use crate::simulation::Object;
use crate::simulation::server_sim::player::components::{Player, PlayerTurret};
use crate::simulation::server_sim::player::{OnPlayerDeathEvent, Health, OnKillEvent, OnHealthChangedEvent};
use crate::simulation::server_sim::bullet::BulletSystemStage::{CollisionHandle, CollisionSend};
use crate::utils::commands::despawn::CustomDespawnExt;

static BULLET_COLLIDER_RADIUS: f32 = 10.;
static BULLET_OFFSET: f32 = 95.;
static BULLET_LIFETIME: f32 = 3.0;
static BULLET_DAMAGE: f32 = 5.0;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BulletCollisionEvent>()
            .configure_set(CollisionSend.before(CollisionHandle))
            .add_systems(
                (
                    fire_bullet.before(CollisionSend),
                    bullet_decay.after(fire_bullet),
                    bullet_collision_sender.in_set(CollisionSend),
                    bullet_collision_handler.in_set(CollisionHandle)
                ).in_set(ServerUpdate)
            );
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
    pub player: Entity,
}

#[derive(SystemSet, Debug, Eq, PartialEq, Hash, Clone)]
pub enum BulletSystemStage {
    CollisionSend,
    CollisionHandle,
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
                AutoSorted,
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
    mut commands: Commands,
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
    players: Query<Entity, With<Player>>,
) {
    collision_events.iter().for_each(|&e| {
        if let CollisionEvent::Started(ent1, ent2, _) = e {
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
    mut kill_writer: EventWriter<OnKillEvent>,
    mut death_writer: EventWriter<OnPlayerDeathEvent>,
    mut health_writer: EventWriter<OnHealthChangedEvent>,
    bullets: Query<&Bullet>,
    mut healths: Query<(Entity, &mut Health)>,
    players: Query<&Player>,
    lobby: Res<Lobby>,
    mut commands: Commands,
) {
    events.iter().for_each(|e| {
        let Ok((entity, mut health)) = healths.get_mut(e.player) else { return; };
        let Ok(bullet) = bullets.get(e.bullet) else { return; };
        health.health = (health.health - bullet.damage).clamp(0., health.max_health);
        if health.health <= 0. {
            commands.entity(entity).custom_despawn();
        }

        if let Some(attacker) = bullet.owner &&
            let Ok(&Player { id: victim_id, .. }) = players.get(entity) {
            health_writer.send(OnHealthChangedEvent {
                object_id: lobby.player_data.get(&victim_id).unwrap().object_id.unwrap(),
                health: health.health,
                max_health: health.max_health,
            });

            if health.health <= 0. {
                death_writer.send(OnPlayerDeathEvent { player_id: victim_id });

                if let Ok(&Player { id: attacker_id, .. }) = players.get(attacker) {
                    kill_writer.send(OnKillEvent {
                        attacker_id,
                        victim_id,
                    });
                }
            }
        }
    })
}