use bevy::app::App;
use bevy::asset::Handle;
use bevy::prelude::{Commands, default, Entity, Image, Plugin, Query, Res, SpriteBundle};
use crate::assets::{GameAssets, SpriteEnum};

pub struct SpriteUpdatePlugin;

impl Plugin for SpriteUpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_sprite_handle);
    }
}

fn update_sprite_handle(
    q: Query<(Entity, &SpriteEnum, Option<&Handle<Image>>)>,
    assets: Res<GameAssets>,
    mut commands: Commands,
) {
    for (ent, &sprite, handle) in q.iter() {
        enum ActionType {
            UpdateHandle,
            InsertHandle,
            Neither,
        }

        let action = match handle {
            Some(handle) => {
                if handle != &assets.get(sprite)
                { ActionType::UpdateHandle } else { ActionType::Neither }
            }
            None => ActionType::InsertHandle,
        };
        match action {
            ActionType::UpdateHandle => { commands.entity(ent).insert(assets.get(sprite)); }
            ActionType::InsertHandle => {
                commands.entity(ent).insert(
                    SpriteBundle {
                        texture: assets.get(sprite),
                        ..default()
                    });
            }
            ActionType::Neither => {}
        };
    }
}
