use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::LdtkAsset;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Menu),
        )
        .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, PlayerWalkTextureAtlasAsset>(GameState::Loading)
        .add_collection_to_loading_state::<_, EnemyTextureAtlasAsset>(GameState::Loading)
        .add_collection_to_loading_state::<_, ClockTextureAtlasAsset>(GameState::Loading)
        .add_collection_to_loading_state::<_, LevelAsset>(GameState::Loading);
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    // #[asset(path = "audio/flying.ogg")]
    // pub flying: Handle<AudioSource>,
    #[asset(path = "audio/main_loop.ogg")]
    pub main_music_loop: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct PlayerWalkTextureAtlasAsset {
    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 3, rows = 1))]
    #[asset(path = "sprites/hook_sheet.png")]
    pub walking: Handle<TextureAtlas>,
}

#[derive(AssetCollection, Resource)]
pub struct EnemyTextureAtlasAsset {
    #[asset(texture_atlas(tile_size_x = 64., tile_size_y = 32., columns = 4, rows = 1))]
    #[asset(path = "sprites/crocodile_sheet.png")]
    pub enemy_atlas: Handle<TextureAtlas>,
}

#[derive(AssetCollection, Resource)]
pub struct ClockTextureAtlasAsset {
    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 6, rows = 1))]
    #[asset(path = "sprites/clock_sheet.png")]
    pub clock_atlas: Handle<TextureAtlas>,
}

#[derive(AssetCollection, Resource)]
pub struct LevelAsset {
    #[asset(path = "sprites/level.ldtk")]
    pub level: Handle<LdtkAsset>,
}
