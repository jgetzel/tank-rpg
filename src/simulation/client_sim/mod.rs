use bevy::prelude::*;
use crate::ClientSet::{ClientReceive, ClientUpdate};
use crate::simulation::client_sim::systems::*;
use crate::utils::networking::is_client_connected;

mod systems;


pub struct ClientSimulationPlugin;

impl Plugin for ClientSimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(on_you_joined.after(ClientReceive).run_if(is_client_connected))
            .add_systems(
                (
                    phys_obj_updater,
                    turr_updater,
                    on_player_join,
                    on_player_leave,
                    on_player_spawn,
                    on_player_update,
                    on_health_update,
                    on_timer_update,
                ).in_set(ClientUpdate).before(on_object_despawn)
            )
            .add_system(on_object_despawn.in_set(ClientUpdate));
    }
}
