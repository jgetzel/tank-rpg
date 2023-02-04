use bevy::prelude::{Query, ResMut, Transform};
use bevy_renet::renet::{DefaultChannel, RenetServer};
use bevy_rapier2d::dynamics::Velocity;
use bevy::utils::HashMap;
use crate::assets::SpriteEnum;
use crate::networking::messages::{PhysicsObjData, UnreliableMessages};
use crate::object::{Object, ObjectId};

pub fn phys_obj_update(
    mut server: ResMut<RenetServer>,
    query: Query<(&Object, &Transform, &Velocity, &SpriteEnum)>,
) {
    let mut objects: HashMap<ObjectId, PhysicsObjData> = HashMap::new();
    for (object, transform, vel, &sprite) in query.iter() {
        objects.insert(object.id, PhysicsObjData {
            translation: transform.translation,
            velocity: vel.linvel,
            sprite
        });
    }
    let sync_msg = bincode::serialize(&UnreliableMessages::PhysObjUpdate { objects })
        .unwrap();
    server.broadcast_message(DefaultChannel::Unreliable, sync_msg);
}