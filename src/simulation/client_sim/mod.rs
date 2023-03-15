use bevy::prelude::*;
use crate::ClientSet::ClientUpdate;

mod systems;


pub struct ClientSimulationPlugin;

impl Plugin for ClientSimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                systems::phys_obj_updater,
                systems::turr_updater,
            ).in_set(ClientUpdate)
        );
    }
}
