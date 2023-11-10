use crate::enemy::Enemy;
use crate::loading::AudioAssets;
use crate::player::Player;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_systems(
                OnEnter(GameState::Playing),
                (start_music, start_enemy_sound_fx),
            )
            .add_systems(OnExit(GameState::Playing), (stop_sound, cleanup).chain())
            .add_systems(
                Update,
                attenuate_ticktock.run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Resource)]
// struct FlyingAudio(Handle<AudioInstance>);
pub struct MainMusicLoop(pub Handle<AudioInstance>);

#[derive(Resource)]
pub struct TickTockLoop(pub Handle<AudioInstance>);

#[derive(Resource)]
pub struct AlarmSoundEffect(pub Handle<AudioInstance>);

fn start_music(mut commands: Commands, audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    //audio.pause();
    let handle = audio
        .play(audio_assets.main_music_loop.clone())
        .looped()
        .with_volume(0.3)
        .paused()
        .handle();
    commands.insert_resource(MainMusicLoop(handle));
}

fn stop_sound(
    main_music: Res<MainMusicLoop>,
    ticktock: Res<TickTockLoop>,
    alarm: Res<AlarmSoundEffect>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Some(instance) = audio_instances.get_mut(&main_music.0) {
        instance.pause(AudioTween::default());
    }

    if let Some(instance) = audio_instances.get_mut(&ticktock.0) {
        instance.pause(AudioTween::default());
    }

    if let Some(instance) = audio_instances.get_mut(&alarm.0) {
        instance.pause(AudioTween::default());
    }
}

/// Play the tick tock loop
fn start_enemy_sound_fx(mut commands: Commands, audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    //audio.pause();
    let ticktock_handle = audio
        .play(audio_assets.tick_tock.clone())
        .looped()
        .with_volume(0.0)
        .handle();

    let alarm_handle = audio
        .play(audio_assets.alarm.clone())
        .paused()
        .with_volume(0.3)
        .handle();

    commands.insert_resource(TickTockLoop(ticktock_handle));
    commands.insert_resource(AlarmSoundEffect(alarm_handle));
}

/// Attenuate the ticktock sound based on the distance between the player and the closest enemy
fn attenuate_ticktock(
    player_query: Query<&GlobalTransform, With<Player>>,
    enemy_query: Query<&GlobalTransform, With<Enemy>>,
    ticktock: Res<TickTockLoop>,
    mut audio_assets: ResMut<Assets<AudioInstance>>,
) {
    const MAX_DISTANCE: f32 = 320.;
    const MAX_VOLUME: f32 = 0.3;

    let player_position = player_query.single().translation();
    let mut distances = Vec::<f32>::new();

    for enemy_position in enemy_query.iter() {
        distances.push(enemy_position.translation().distance(player_position));
    }

    distances.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let shortest_distance = distances[0];

    let volume = ((MAX_DISTANCE - shortest_distance) / MAX_DISTANCE).clamp(0.0, 1.0) * MAX_VOLUME;

    if let Some(instance) = audio_assets.get_mut(&ticktock.0) {
        instance.set_volume(volume as f64, AudioTween::default());
    }
}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<MainMusicLoop>();
    commands.remove_resource::<TickTockLoop>();
    commands.remove_resource::<AlarmSoundEffect>();
}
