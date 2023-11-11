use std::default;

use bevy::{
    asset::diagnostic::AssetCountDiagnosticsPlugin,
    diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_editor_pls::prelude::*;
use bevy_inspector_egui::egui::ahash::HashSet;

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
