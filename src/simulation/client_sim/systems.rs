use crate::asset_loader::components::SpriteEnum;
use crate::simulation::Object;
use crate::simulation::SyncedObjects;
use bevy::prelude::{Children, Commands, default, Entity, EventReader, Query, Res, ResMut, SpriteBundle, Transform, With};
use bevy_rapier2d::dynamics::Velocity;
use crate::asset_loader::resources::SpriteAssets;
use crate::client_networking::{RecvPhysObjUpdateEvent, RecvTurretUpdateEvent};
use crate::simulation::server_sim::player::{Player, PlayerTurret};

#[allow(clippy::type_complexity)]
pub fn phys_obj_updater(
    mut update_event: EventReader<RecvPhysObjUpdateEvent>,
    mut query: Query<(&mut Transform,
                      Option<&mut Velocity>,
                      Option<&mut SpriteEnum>),
        With<Object>>,
    mut objects: ResMut<SyncedObjects>,
    assets: Res<SpriteAssets>,
    mut commands: Commands,
) {
    update_event.iter().for_each(|ev| match objects.objects.get(&ev.id) {
            Some(&entity) => {
                if let Ok((mut trans, vel, sprite)) = query.get_mut(entity) {
                    trans.translation = ev.data.translation;
                    if let Some(mut vel) = vel {
                        vel.linvel = ev.data.velocity;
                    }
                    match sprite {
                        Some(mut sprite) => *sprite = ev.data.sprite,
                        None => {
                            commands.entity(entity).insert(ev.data.sprite);
                        }
                    }
                }
            }
            None => {
                objects.objects.insert(ev.id, init_object(ev, &mut commands, &assets));
            }
        });
}

pub fn turr_updater(
    mut update_event: EventReader<RecvTurretUpdateEvent>,
    objects: ResMut<SyncedObjects>,
    parent_q: Query<&Children, With<Player>>,
    mut turr_q: Query<&mut Transform, With<PlayerTurret>>
) {
    update_event.iter().for_each(|ev| {
        if let Some(&ent) = objects.objects.get(&ev.parent_id) {
            if let Ok(children) = parent_q.get(ent) {
                children.iter().for_each(|&child| {
                    if let Ok(mut turr) = turr_q.get_mut(child) {
                        turr.rotation = ev.rotation;
                    }
                });
            }
        }
    })
}

fn init_object(event: &RecvPhysObjUpdateEvent, commands: &mut Commands, assets: &SpriteAssets) -> Entity {
    commands
        .spawn((
            SpriteBundle {
                texture: assets.get(event.data.sprite),
                transform: Transform {
                    translation: event.data.translation,
                    ..default()
                },
                ..default()
            },
            Velocity {
                linvel: event.data.velocity,
                ..default()
            },
            Object { id: event.id },
        ))
        .id()
}
