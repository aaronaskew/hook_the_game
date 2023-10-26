#![allow(unused)]
use crate::{player::Player, *};
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, render::primitives::Aabb};
use bevy_debug_text_overlay::{screen_print, OverlayPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_2d::prelude::*;

/// Debugging tools
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // FrameTimeDiagnosticsPlugin,
            WorldInspectorPlugin::new(),
            OverlayPlugin::default(),
        ))
        .add_systems(Update, Self::show_state)
        .add_systems(
            Update,
            Self::player_info.run_if(in_state(GameState::Playing)),
        );
    }
}

impl DebugPlugin {
    fn player_info(
        _query: Query<(&Transform, Option<&Name>)>,
        _state: Res<State<GameState>>,
        _ortho: Query<&OrthographicProjection>,
        player_physics: Query<
            (
                &Aabb,
                &Collider,
                &ColliderAabb,
                &Position,
                &LinearVelocity,
                &ExternalForce,
                &CollidingEntities,
            ),
            With<Player>,
        >,
        time: Res<Time>,
    ) {
        let current_time = time.elapsed_seconds_f64();
        let at_interval = |t: f64| current_time % t < time.delta_seconds_f64();

        if at_interval(0.01) {
            player_physics.iter().for_each(
                |(aabb, coll, coll_aabb, pos, vel, force, colliding_ents)| {
                    //screen_print!("cam scale {:?}", _ortho.single().scale);
                    //screen_print!("pos {:?}", pos);
                },
            );
        }
    }

    fn show_state(state: Res<State<GameState>>, time: Res<Time>) {
        let current_time = time.elapsed_seconds_f64();
        let at_interval = |t: f64| current_time % t < time.delta_seconds_f64();

        if at_interval(2.0) {
            //screen_print!("state: {:?}", state);
        }
    }
}
