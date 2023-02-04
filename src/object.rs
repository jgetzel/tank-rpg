use std::sync::atomic::{AtomicU64, Ordering};
use bevy::prelude::{Commands, Component, default, Entity, EventReader, Query, Res, Resource, SpriteBundle, Transform, With};
use bevy_rapier2d::dynamics::Velocity;
use bevy::utils::HashMap;
use crate::assets::{GameAssets};
use crate::networking::client::{PhysObjUpdateEvent};

static COUNTER: AtomicU64 = AtomicU64::new(0);

pub type ObjectId = u64;

#[derive(Component)]
pub struct Object {
    pub id: ObjectId
}

impl Object {
    pub fn new() -> Self {
        Object {
            id: COUNTER.fetch_add(1, Ordering::Relaxed)
        }
    }
}

impl Default for Object {
    fn default() -> Self {
        Object::new()
    }
}

#[derive(Debug, Default, Resource)]
pub struct SyncedObjects {
    pub objects: HashMap<ObjectId, Entity>,
}

pub fn phys_obj_updater(
    mut update_event: EventReader<PhysObjUpdateEvent>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Object>>,
    objects: Res<SyncedObjects>,
    assets: Res<GameAssets>,
    mut commands: Commands
) {
    for ev in update_event.iter() {
        match objects.objects.get(&ev.id) {
            Some(&entity) => {
                if let Ok((mut trans, mut vel)) = query.get_mut(entity) {
                    trans.translation = ev.data.translation;
                    vel.linvel = ev.data.velocity;
                }
            },
            None => { init_object(ev, &mut commands, &assets); }
        }
    }
}

pub fn init_object(event: &PhysObjUpdateEvent, commands: &mut Commands, assets: &GameAssets) {
    commands.spawn((
        SpriteBundle {
            texture: assets.map.get(&event.data.sprite).unwrap().clone(),
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
    ));

    // for player in join_ev.iter().map(|e| e.player_id) {
    //     if let Some(&player_entity) = lobby.players.get(&player) {
    //         commands.entity(player_entity).with_children(|p| {
    //             p.spawn(SpriteBundle {
    //                 texture: assets.tank_gray.clone(),
    //                 ..default()
    //             });
    //         });
    //         debug!("Sprite added for Player {player}");
    //     }
    // }
}
