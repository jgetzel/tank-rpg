use bevy::prelude::Vec2;
use crate::simulation::server_sim::player::components::PlayerInput;
use crate::simulation::server_sim::player::Player;

pub fn calc_player_next_velocity(
    curr_vel: Vec2,
    player: &Player,
    input: &PlayerInput,
    delta_time: f32,
) -> Vec2 {
    let new_velocity = curr_vel + (player.accel * input.movement * delta_time);
    if [player.max_speed, curr_vel.length()].iter().all(|v| new_velocity.length() > *v) {
        new_velocity.clamp_length_max(player.max_speed)
    } else {
        new_velocity
    }
}
