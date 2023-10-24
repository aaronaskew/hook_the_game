#![allow(unused_variables)]
/// Debugging tools
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldInspectorPlugin::new())
            .add_systems(Update, debug_update);
    }
}

fn debug_update(
    mut _commands: Commands,
    _query: Query<(&Transform, Option<&Name>)>,
    _state: Res<State<GameState>>,
) {
    // console_log!("debug_update start");
    // console_log!("state: {:?}, got {} Transforms", state, query.iter().len());

    // for (i, (t, n)) in query.iter().enumerate() {
    //     console_log!("idx: {} name: {:?} transform: {:?}", i, n, t,);
    // }

    // console_log!("debug_update end");
}
