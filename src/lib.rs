#![allow(clippy::type_complexity)]
use bevy::app::App;
use bevy::prelude::*;

mod actions;
mod audio;
#[macro_use]
mod utils;
mod background;
mod camera;
#[cfg(debug_assertions)]
mod debug;
mod enemy;
mod level;
mod loading;
mod menu;
mod physics;
mod player;
mod shader_utils;
mod video;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::background::BackgroundPlugin;
use crate::camera::CameraPlugin;
#[cfg(debug_assertions)]
use crate::debug::DebugPlugin;
use crate::enemy::EnemyPlugin;
use crate::level::LevelPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::physics::PhysicsPlugin;
use crate::player::PlayerPlugin;
use crate::video::VideoPlugin;

pub const GAME_SIZE: Vec2 = Vec2 { x: 1600., y: 900. };

///This example game uses States to separate logic
///See https://bevy-cheatbook.github.io/programming/states.html
///Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    /// Do game initialization
    #[default]
    Initializing,
    /// During the loading State the `LoadingPlugin` will load our assets.
    /// #### Transitions:
    /// - `Loading` => `Menu`
    Loading,
    /// Here the menu is drawn and waiting for player interaction
    /// #### Transitions:
    /// - Any => `Menu`
    /// - `Menu` => `BuildingLevel`
    Menu,

    /// Builds the level
    /// #### Transitions:
    /// - Any => `BuildingLevel`
    /// - `BuildingLevel` => `SpawningPlayer`
    LoadingLevel,

    /// During this State the player is spawned
    /// - `BuildingLevel` => `SpawningPlayer`
    /// - `SpawningPlayer` => `InitializingPhysics`
    SpawningEntities,

    /// Initializes physics
    /// - `SpawningPlayer` => `InitializingPhysics`
    /// - `InitializingPhysics` => `Playing`
    InitializingPhysics,

    /// During this State the actual game logic is executed
    /// - `InitializingPhysics` => `Playing`
    /// - `Playing` => `PlayingCutScene`
    Playing,

    /// Here the cutscene is played
    /// - `Playing` => `PlayingCutScene`
    /// - `PlayingCutScene` => `Menu`
    PlayingCutScene,
}

#[derive(Component, Clone, Reflect)]
pub struct AnimationTimer {
    pub timer: Timer,
    pub frame_count: usize,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Hook - The Game".to_string(),
                        resolution: (GAME_SIZE.x, GAME_SIZE.y).into(),
                        // Bind to canvas included in `index.html`
                        canvas: Some("#bevy".to_owned()),
                        // Tells wasm not to override default event handling, like F5 and Ctrl+R
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            LoadingPlugin,
            MenuPlugin,
            ActionsPlugin,
            InternalAudioPlugin,
            PlayerPlugin,
            VideoPlugin,
            PhysicsPlugin,
            LevelPlugin,
            BackgroundPlugin,
            EnemyPlugin,
            CameraPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins(DebugPlugin);
        }
    }
}
