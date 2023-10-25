#![allow(unused)]
/// Debugging tools
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::{loading::PlayerWalk, *};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldInspectorPlugin::new())
            .add_systems(OnEnter(GameState::Playing), debug_update);
    }
}

fn debug_update(
    mut commands: Commands,
    _query: Query<(&Transform, Option<&Name>)>,
    _state: Res<State<GameState>>,
    _ortho: Query<&OrthographicProjection>,
    player_walk: Res<PlayerWalk>,
) {

    { // debug messages
         // console_log!("debug_update start");
         // console_log!("state: {:?}, got {} Transforms", state, query.iter().len());

        // for (i, (t, n)) in query.iter().enumerate() {
        //     console_log!("idx: {} name: {:?} transform: {:?}", i, n, t,);
        // }

        // console_log!("debug_update end");
    }
}
