use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::prelude::{Commands, EventReader, EventWriter, PluginGroup, Query, Res, ResMut, SystemSet, Transform, WindowDescriptor, With};
use bevy::sprite::SpriteBundle;
use bevy::utils::default;
use bevy::window::WindowPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::parry::transformation::utils::transform;
use bevy_renet::{RenetClientPlugin, run_if_client_connected};
use bevy_renet::renet::RenetClient;
use tank_rpg::assets::{AppState, AssetsLoading, check_assets_loaded, GameAssets, load_assets};
use tank_rpg::camera::{camera_move, init_camera};
use tank_rpg::environment::init_background;
use tank_rpg::input_helper::{keyboard_events, mouse_position, PlayerInput};
use tank_rpg::networking::{client, Lobby};
use tank_rpg::networking::client::{PlayerJoinEvent, PlayerLeaveEvent, PlayerUpdateEvent};
use tank_rpg::player::{Player, You};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "Client Window".into(),
            ..default()
        },
        ..default()
    }))
        .add_plugin(RenetClientPlugin::default())
        .add_state(AppState::Loading)
        .insert_resource(client::new_client())
        .insert_resource(Lobby::default())
        .insert_resource(GameAssets::default())
        .insert_resource(AssetsLoading::default())
        .insert_resource(PlayerInput::default())
        .add_system_set(
            SystemSet::on_enter(AppState::Loading)
                .with_system(load_assets)
        )
        .add_system_set(
            SystemSet::on_update(AppState::Loading)
                .with_system(check_assets_loaded)
        )
        .add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(init_background)
                .with_system(init_camera)
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(camera_move)
        )
        .add_system(keyboard_events)
        .add_system(mouse_position)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(run_if_client_connected)
                .with_system(client::client_send_input)
                .with_system(client::client_recv)
        );

    app.add_event::<PlayerJoinEvent>()
        .add_event::<PlayerLeaveEvent>()
        .add_event::<PlayerUpdateEvent>()
        .add_system(you_tag_adder)
        .add_system(player_sprite_spawner)
        .add_system(player_pos_updater);

    app.add_plugin(WorldInspectorPlugin);

    app.run();
}

fn player_sprite_spawner(
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
        }
    }
}

fn you_tag_adder(
    mut join_event: EventReader<PlayerJoinEvent>,
    mut commands: Commands,
    client: Res<RenetClient>,
    lobby: Res<Lobby>
) {
    for ev in join_event.iter() {
        if ev.player_id == client.client_id() {
            if let Some(&player_entity) = lobby.players.get(&ev.player_id) {
                commands.entity(player_entity).insert(You);
            }
        }
    }
}

fn player_pos_updater(
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