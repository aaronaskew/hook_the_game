#![allow(unused)]
/// Debugging tools
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::{Collider, RigidBody};

use crate::{loading::PlayerWalk, player::AnimationTimer, *};

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
    let _ortho = _ortho.single();

    let window_size = _ortho.area.as_rect();

    let sprite = TextureAtlasSprite {
        //custom_size: Some(Vec2::splat(140.)),
        ..default()
    };

    for i in 0..100 {
        let sprite = sprite.clone();

        commands
            .spawn((SpriteSheetBundle {
                texture_atlas: player_walk.walking.clone(),
                sprite,
                transform: Transform::from_xyz(
                    rand::random::<f32>() * window_size.width() as f32
                        - window_size.width() as f32 * 0.5,
                    rand::random::<f32>() * window_size.height() as f32
                        - window_size.height() as f32 * 0.5,
                    0.0,
                ),
                ..default()
            },))
            .insert(Name::new(format!("dummy_{}", i)))
            .insert(RigidBody::Dynamic)
            .insert(Collider::cuboid(5.0, 16.0));
    }

    // console_log!("debug_update start");
    // console_log!("state: {:?}, got {} Transforms", state, query.iter().len());

    // for (i, (t, n)) in query.iter().enumerate() {
    //     console_log!("idx: {} name: {:?} transform: {:?}", i, n, t,);
    // }

    // console_log!("debug_update end");
}
