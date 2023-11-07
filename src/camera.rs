use bevy::prelude::*;

use crate::{player::Player, GameState};

/// The amount to zoom the scale the camera projection (lower = zoom in)
pub const CAMERA_ZOOM: f32 = 0.25;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Initializing),
            (spawn_camera_and_next_state).chain(),
        )
        .add_systems(
            OnExit(GameState::Initializing),
            (scale_camera_projection).chain(),
        )
        .add_systems(Update, follow_player.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
struct MainCamera;

fn spawn_camera_and_next_state(mut commands: Commands, mut state: ResMut<NextState<GameState>>) {
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);
    state.set(GameState::Loading);
}

fn scale_camera_projection(mut q: Query<&mut OrthographicProjection, With<MainCamera>>) {
    let mut projection = q.single_mut();

    // scale camera
    projection.scale *= CAMERA_ZOOM;
    projection.scale = projection.scale.clamp(0.25, 5.0);
}

// fn recenter_camera(mut transform_q: Query<&mut Transform, With<MainCamera>>) {
//     let mut transform = transform_q.single_mut();

//     let scaled_camera_transform = GAME_SIZE * CAMERA_ZOOM / 2.0;

//     // recenter camera
//     transform.translation = Vec3::new(scaled_camera_transform.x, scaled_camera_transform.y, 0.0);
// }

fn follow_player(
    mut camera: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    player: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let follow_sharpness = 0.1;
    let blend = 1.0 - (1.0_f32 - follow_sharpness).powf(time.delta_seconds() * 30.0);

    let mut camera_transform = camera.single_mut();
    let player_transform = *player.single();

    camera_transform.translation = camera_transform
        .translation
        .lerp(player_transform.translation, blend);
}
