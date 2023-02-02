use bevy::log::{debug, info};
use bevy::math::{Quat, Vec2, Vec3};
use bevy::prelude::{BuildChildren, Commands, Component, default, Entity, EventReader, GlobalTransform, Query, Res, ResMut, SpriteBundle, Time, Transform, With};
use bevy_rapier2d::dynamics::Velocity;
use crate::assets::GameAssets;
use crate::input_helper::PlayerInput;
use crate::networking::client::{PlayerJoinEvent, PlayerUpdateEvent};
use crate::networking::Lobby;

pub mod bundles;

#[derive(Component)]
pub struct You;

#[derive(Component, Clone)]
pub struct Player {
    pub id: u64,
    pub accel: f32,
    pub max_speed: f32,
    pub friction: f32,
    pub curr_velocity: Vec2,
}

impl Player {
    fn new(id: u64) -> Self {
        Player {
            id,
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

pub fn init_player(
    mut join_ev: EventReader<PlayerJoinEvent>,
    mut commands: Commands,
    mut lobby: ResMut<Lobby>
) {
    for ev in join_ev.iter() {
        info!("Player {} Connected", ev.player_id);
        let player_entity = spawn_new_player(&mut commands, ev.player_id, None);
        lobby.players.insert(ev.player_id,player_entity);
    }
}

pub fn player_move(
    mut query: Query<(&mut Velocity, &Player, &PlayerInput)>,
    time: Res<Time>,
) {
    for (mut velocity, player, input) in query.iter_mut() {
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
    input: Res<PlayerInput>,
    mut query: Query<(&mut Transform, &GlobalTransform, &mut PlayerTurret)>,
) {
    for (mut trans, global_trans, mut turret) in query.iter_mut() {
        let diff = input.mouse_position - global_trans.translation().truncate();
        let angle = diff.y.atan2(diff.x);
        trans.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);

        turret.direction = diff.normalize();
    }
}

pub fn  spawn_new_player(commands: &mut Commands, id: u64, pos: Option<Vec2>) -> Entity {
    commands.spawn(bundles::get_player_bundle(id, pos))
        .with_children(|p| {
            p.spawn(bundles::get_turret_bundle());
        }).id()
}

pub fn player_sprite_spawner(
    mut join_event: EventReader<PlayerJoinEvent>,
    mut commands: Commands,
    lobby: ResMut<Lobby>,
    assets: Res<GameAssets>,
    query: Query<&mut Transform, With<Player>>
) {
    for player in join_event.iter().map(|e| e.player_id) {
        if let Some(&player_entity) = lobby.players.get(&player) {
            commands.entity(player_entity).insert(SpriteBundle {
                texture: assets.tank_gray.clone(),
                transform: *query.get(player_entity).unwrap_or(&Transform::default()),
                ..default()
            });
            debug!("Sprite added for Player {player}");
        }
    }
}

pub fn player_pos_updater(
    mut update_event: EventReader<PlayerUpdateEvent>,
    mut query: Query<&mut Transform, With<Player>>,
    lobby: Res<Lobby>,
) {
    for ev in update_event.iter() {
        if let Some(&entity) = lobby.players.get(&ev.player_id) {
            if let Ok(mut trans) = query.get_mut(entity) {
                trans.translation = ev.translation;
            }
        }
    }
}
