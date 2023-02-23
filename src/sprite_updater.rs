use bevy::app::App;
use bevy::asset::Handle;
use bevy::prelude::{Commands, default, Entity, Image, Plugin, Query, Res, Sprite, SpriteBundle, Transform};
use crate::asset_loader::components::SpriteEnum;
use crate::asset_loader::resources::SpriteAssets;

pub struct SpriteUpdatePlugin;

impl Plugin for SpriteUpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_sprite_handle);
    }
}

#[allow(clippy::type_complexity)]
fn update_sprite_handle(
    q: Query<(Entity, &SpriteEnum, Option<&Handle<Image>>, Option<&Sprite>, Option<&Transform>)>,
    assets: Res<SpriteAssets>,
    mut commands: Commands,
) {
    q.iter().for_each(|(ent, &sprite_enum, handle, sprite_info, trans, )| {
        enum ActionType {
            UpdateHandle,
            InsertBundle,
            Neither,
        }

        let action = match handle {
            Some(handle) => {
                if handle != &assets.get(sprite_enum)
                { ActionType::UpdateHandle } else { ActionType::Neither }
            }
            None => ActionType::InsertBundle,
        };
        match action {
            ActionType::UpdateHandle => { commands.entity(ent).insert(assets.get(sprite_enum)); }
            ActionType::InsertBundle => {
                let mut ent_commands = commands.entity(ent);
                let ent_commands = ent_commands.insert(
                    SpriteBundle {
                        texture: assets.get(sprite_enum),
                        ..default()
                    });

                if let Some(sprite_info) = sprite_info {
                    ent_commands.insert(sprite_info.clone());
                }
                if let Some(&trans) = trans {
                    ent_commands.insert(trans);
                }
            }
            ActionType::Neither => {}
        };
    });
}
