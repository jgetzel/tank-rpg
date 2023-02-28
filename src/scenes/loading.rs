use bevy::app::App;
use bevy::prelude::{EventReader, Plugin, ResMut, State, SystemSet};
use crate::asset_loader::AssetsLoadedEvent;
use crate::scenes::AppState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::Loading)
                .with_system(assets_loaded_reader)
        );
    }
}

fn assets_loaded_reader(
    mut events: EventReader<AssetsLoadedEvent>,
    mut state: ResMut<State<AppState>>,
) {
    events.iter().for_each(|_| {
        state.set(AppState::MainMenu).unwrap();
    });
}