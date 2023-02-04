use bevy::log::info;
use bevy::math::{Quat, Vec2, Vec3};
use bevy::prelude::{BuildChildren, Commands, Component, Entity, EventReader, Query, Res, ResMut, Time, Transform};
use bevy_rapier2d::dynamics::Velocity;
use crate::input_helper::PlayerInput;
use crate::networking::client::PlayerJoinEvent;
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
    mut lobby: ResMut<Lobby>,
) {
    for ev in join_ev.iter() {
        info!("Player {} Connected", ev.player_id);
        let player_entity = spawn_new_player(&mut commands, ev.player_id, None);
        lobby.players.insert(ev.player_id, player_entity);
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
    mut query: Query<(&mut Transform, &mut PlayerTurret)>,
) {
    for (mut trans, mut turret) in query.iter_mut() {
        turret.direction = input.turret_dir;

        let angle = input.turret_dir.y.atan2(input.turret_dir.x);
        trans.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);
    }
}

pub fn spawn_new_player(commands: &mut Commands, id: u64, pos: Option<Vec2>) -> Entity {
    commands.spawn(bundles::get_player_bundle(id, pos))
        .with_children(|p| {
            p.spawn(bundles::get_turret_bundle());
        }).id()
}
