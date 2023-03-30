use bevy::prelude::{Children, GlobalTransform, Query, Res, Time, Transform, With};
use bevy_rapier2d::dynamics::Velocity;
use bevy::math::{Quat, Vec3};
use crate::simulation::server_sim::player::components::PlayerInput;
use crate::simulation::server_sim::player::{Player, PlayerTurret};
use crate::simulation::server_sim::player::movement_state_machine::MovementStateMachine;
use crate::utils::math::state_machine::StateMachine;

pub fn player_turret_rotate(
    player_q: Query<(&Children, &PlayerInput), With<Player>>,
    mut turr_q: Query<(&mut Transform, &GlobalTransform, &mut PlayerTurret)>,
) {
    player_q.iter().for_each(|(children, input)| {
        children.iter().for_each(|&child| {
            if let Ok((mut trans, glob_trans, mut turr)) = turr_q.get_mut(child) {
                turr.direction = (input.mouse_pos - glob_trans.translation().truncate()).normalize();

                let angle = turr.direction.y.atan2(turr.direction.x);
                trans.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);
            }
        });
    });
}

pub fn movement_machine_update(
    mut query: Query<(&mut Velocity, &Player, &PlayerInput, &mut MovementStateMachine)>,
    time: Res<Time>,
) {
    query.iter_mut().for_each(|(mut vel, player, input, mut sm)| {
        sm.update( (player, input, &mut vel, time.delta_seconds()));
    });
}

