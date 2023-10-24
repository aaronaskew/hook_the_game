use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            // TODO skipping menu for now
            LoadingState::new(GameState::Loading).continue_to_state(GameState::SpawningPlayer),
        )
        .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, PlayerWalk>(GameState::Loading)
        .add_systems(OnExit(GameState::Loading), spawn_camera);
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

// #[derive(AssetCollection, Resource)]
// pub struct TextureAssets {
//     #[asset(path = "textures/bevy.png")]
//     pub texture_bevy: Handle<Image>,

// }

#[derive(AssetCollection, Resource)]
pub struct PlayerWalk {
    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 3, rows = 1))]
    #[asset(path = "sprites/hook_sheet.png")]
    pub walking: Handle<TextureAtlas>,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
