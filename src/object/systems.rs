use bevy::asset::Handle;
use bevy::prelude::{Commands, default, Entity, EventReader, Image, Query, Res, ResMut, SpriteBundle, Transform, With};
use bevy_rapier2d::dynamics::Velocity;
use crate::assets::{GameAssets, SpriteEnum};
use crate::networking::PhysObjUpdateEvent;
use crate::object::SyncedObjects;
use crate::object::components::Object;

pub fn phys_obj_updater(
    mut update_event: EventReader<PhysObjUpdateEvent>,
    mut query: Query<(&mut Transform, &mut Velocity, Option<&mut Handle<Image>>, Option<&SpriteEnum>), With<Object>>,
    mut objects: ResMut<SyncedObjects>,
    assets: Res<GameAssets>,
    mut commands: Commands,
) {
    for ev in update_event.iter() {
        match objects.objects.get(&ev.id) {
            Some(&entity) => {
                if let Ok((mut trans, mut vel, handle, sprite)) = query.get_mut(entity) {
                    let update_sprite = match handle {
                        Some(_) => {
                            match sprite {
                                Some(&sprite) => sprite != ev.data.sprite,
                                None => true,
                            }
                        }
                        None => true,
                    };
                    if update_sprite {
                        commands.entity(entity).insert(
                            SpriteBundle {
                                texture: assets.get(ev.data.sprite),
                                ..default()
                            });
                    };
                    trans.translation = ev.data.translation;
                    vel.linvel = ev.data.velocity;
                }
            }
            None => {
                objects.objects.insert(ev.id, init_object(ev, &mut commands, &assets));
            }
        }
    }
}

fn init_object(
    event: &PhysObjUpdateEvent,
    commands: &mut Commands,
    assets: &GameAssets,
) -> Entity {
    commands.spawn((
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
        Object { id: event.id }
    )).id()

    // for player in join_ev.iter().map(|e| e.player_id) {
    //     if let Some(&player_entity) = lobby.players.get(&player) {
    //         commands.entity(player_entity).with_children(|p| {
    //             p.spawn(SpriteBundle {
    //                 texture: assets.map.get(&TankGray).unwrap().clone(),
    //                 ..default()
    //             });
    //         });
    //         debug!("Sprite added for Player {player}");
    //     }
    // }
}
