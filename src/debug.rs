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
    mut commands: Commands,
    query: Query<(&Transform, Option<&Name>)>,
    state: Res<State<GameState>>,
) {
    for (transform, name) in query.iter() {
        console_log!(
            "state: {:?} name: {:?} transform: {:?}",
            state,
            name,
            transform,
        );
    }
}
