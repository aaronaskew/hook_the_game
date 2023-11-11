#![allow(unused)]
use crate::{audio::MainMusicLoop, player::Player, *};
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, render::primitives::Aabb};
use bevy_debug_text_overlay::{screen_print, OverlayPlugin};
use bevy_kira_audio::{AudioInstance, AudioTween, PlaybackState};
use bevy_rapier2d::prelude::*;

mod editor;

/// Debugging tools
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // FrameTimeDiagnosticsPlugin,
            editor::HookEditorPlugin,
            OverlayPlugin::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_systems(Update, Self::show_state)
        .add_systems(
            Update,
            (
                Self::player_info.run_if(in_state(GameState::Playing)),
                Self::debug_input.run_if(in_state(GameState::Playing)),
            ),
        );
    }
}

impl DebugPlugin {
    
     fn player_info(
        _query: Query<(&Transform, Option<&Name>)>,
        _state: Res<State<GameState>>,
        _ortho: Query<&OrthographicProjection>,
        player_physics: Query<
            (&Collider, &GlobalTransform, &Velocity, &CollidingEntities),
            With<Player>,
        >,
        time: Res<Time>,
    ) {
        let current_time = time.elapsed_seconds_f64();
        let at_interval = |t: f64| current_time % t < time.delta_seconds_f64();

        if at_interval(0.01) {
            player_physics
                .iter()
                .for_each(|(coll, pos, vel, colliding_entities)| {
                    
                    //detect collisions with player

                    
                    
                    //screen_print!("cam scale {:?}", _ortho.single().scale);
                    //screen_print!("pos {:?}", pos);
                    info!("player colliding entities len: {}", colliding_entities.len());
                    info!("=========================");
                    colliding_entities.iter().for_each(|e| {
                        info!("{:?}", e);
                    });
                });
        }
    }

    fn show_state(state: Res<State<GameState>>, time: Res<Time>) {
        let current_time = time.elapsed_seconds_f64();
        let at_interval = |t: f64| current_time % t < time.delta_seconds_f64();

        if at_interval(2.0) {
            //screen_print!("state: {:?}", state);
        }
    }

    fn debug_input(
        keyboard_input: Res<Input<KeyCode>>,
        music: Res<MainMusicLoop>,
        mut audio_instances: ResMut<Assets<AudioInstance>>,
    ) {
        // able to stop music with `Q`
        if keyboard_input.just_pressed(KeyCode::Q) {
            //toggle music

            if let Some(instance) = audio_instances.get_mut(&music.0) {
                match instance.state() {
                    PlaybackState::Paused { .. } => {
                        instance.resume(AudioTween::default());
                    }
                    PlaybackState::Playing { .. } => {
                        instance.pause(AudioTween::default());
                    }
                    _ => (),
                }
            }
        }
    }

    // fn control_flying_sound(
    //     actions: Res<Actions>,
    //     audio: Res<FlyingAudio>,
    //     mut audio_instances: ResMut<Assets<AudioInstance>>,
    // ) {
    //     if let Some(instance) = audio_instances.get_mut(&audio.0) {
    //         match instance.state() {
    //             PlaybackState::Paused { .. } => {
    //                 if actions.player_movement.is_some() {
    //                     instance.resume(AudioTween::default());
    //                 }
    //             }
}
