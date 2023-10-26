#![allow(clippy::type_complexity)]

mod actions;
mod audio;
#[macro_use]
mod utils;
mod debug; //TODO make this dynamic based on build
mod level;
mod loading;
mod menu;
mod physics;
mod player;
mod video;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::debug::DebugPlugin;
use crate::level::LevelPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::physics::PhysicsPlugin;
use crate::player::PlayerPlugin;
use crate::video::VideoPlugin;

use bevy::app::App;
use bevy::prelude::*;

///This example game uses States to separate logic
///See https://bevy-cheatbook.github.io/programming/states.html
///Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    /// During the loading State the `LoadingPlugin` will load our assets.
    /// #### Transitions:
    /// - Default => `Loading`
    /// - `Loading` => `Menu`
    #[default]
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
    BuildingLevel,

    /// During this State the player is spawned
    /// - `BuildingLevel` => `SpawningPlayer`
    /// - `SpawningPlayer` => `InitializingPhysics`
    SpawningPlayer,

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

#[derive(Component)]
pub struct AnimationTimer {
    pub timer: Timer,
    pub frame_count: usize,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugins((
            LoadingPlugin,
            MenuPlugin,
            ActionsPlugin,
            InternalAudioPlugin,
            PlayerPlugin,
            VideoPlugin,
            PhysicsPlugin,
            LevelPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins(DebugPlugin);
        }
    }
}
