use std::default;

use bevy::{
    asset::diagnostic::AssetCountDiagnosticsPlugin,
    diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    prelude::{Plugin, *},
};
use bevy_editor_pls::prelude::*;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;

use crate::loading::AudioAssets;

pub struct HookEditorPlugin;

impl Plugin for HookEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EditorPlugin::default(),
            FrameTimeDiagnosticsPlugin,
            EntityCountDiagnosticsPlugin,
        ));
    }
}
