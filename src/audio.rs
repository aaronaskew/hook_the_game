// use crate::actions::{set_movement_actions, Actions};
use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_systems(OnEnter(GameState::Playing), start_audio)
            .add_systems(OnExit(GameState::Playing), stop_music);
        // .add_systems(
        //     Update,
        //     control_flying_sound
        //         .after(set_movement_actions)
        //         .run_if(in_state(GameState::Playing)),
    }
}

#[derive(Resource)]
// struct FlyingAudio(Handle<AudioInstance>);
pub struct MainMusicLoop(pub Handle<AudioInstance>);

fn start_audio(mut commands: Commands, audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    //audio.pause();
    let handle = audio
        .play(audio_assets.main_music_loop.clone())
        .looped()
        .with_volume(0.3)
        .handle();
    commands.insert_resource(MainMusicLoop(handle));
}

fn stop_music(audio: Res<MainMusicLoop>, mut audio_instances: ResMut<Assets<AudioInstance>>) {
    if let Some(instance) = audio_instances.get_mut(&audio.0) {
        instance.pause(AudioTween::default());
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
//             PlaybackState::Playing { .. } => {
//                 if actions.player_movement.is_none() {
//                     instance.pause(AudioTween::default());
//                 }
//             }
//             _ => {}
//         }
//     }
// }
