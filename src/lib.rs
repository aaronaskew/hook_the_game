#![allow(clippy::type_complexity)]

mod actions;
mod audio;
#[macro_use]
mod utils;
mod debug; // TODO make this dynamic based on build
mod loading;
mod menu;
mod player;
mod video;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::debug::DebugPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;
use crate::video::VideoPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // Here the menu is drawn and waiting for player interaction
    Menu,
    // During this State the player is spawned
    SpawningPlayer,
    // During this State the actual game logic is executed
    Playing,
    // Here the cutscene is played
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
            DebugPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}
