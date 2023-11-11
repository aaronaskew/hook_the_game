use bevy::prelude::*;
use bevy_kira_audio::{AudioInstance, AudioTween};
use bevy_rapier2d::prelude::*;
use rand::random;

use crate::{audio::AlarmSoundEffect, loading::ClockTextureAtlasAsset, physics, player::Player};

#[derive(Component)]
pub struct Clock {
    pub lifetime: f32,
}

#[derive(Bundle)]
pub struct ClockBundle {
    pub clock: Clock,
    pub physics: physics::bundles::ClockPhysicsBundle,
}

impl Default for ClockBundle {
    fn default() -> Self {
        Self {
            clock: Clock { lifetime: 5.0 },
            physics: physics::bundles::ClockPhysicsBundle::default(),
            
        }
    }
}

#[derive(Component, Clone, Reflect)]
pub struct SpewClocks {
    pub source_position: Vec2,
    pub velocity: Vec2,
    pub rate_interval: f64,
    pub played_sound: bool,
}

pub fn update_clocks(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Clock)>,
) {
    for (entity, mut clock) in query.iter_mut() {
        clock.lifetime -= time.delta_seconds();
        if clock.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn check_collisions_with_player(
    mut query: Query<&CollidingEntities, With<Clock>>,
    mut player_query: Query<(Entity, &mut Player)>,
) {
    let (player_entity, mut player) = player_query.single_mut();

    for entities in query.iter_mut() {
        for entity in entities.iter() {
            if entity == player_entity {
                player.is_alive = false;
            }
        }
    }
}

pub fn spew_clocks(
    mut commands: Commands,
    mut query: Query<(Entity, &mut SpewClocks)>,
    time: Res<Time>,
    clock: Res<ClockTextureAtlasAsset>,
    ticktock: Res<AlarmSoundEffect>,
    mut audio_assets: ResMut<Assets<AudioInstance>>,
) {
    for (entity, mut spew) in query.iter_mut() {
        let current_time = time.elapsed_seconds_f64();
        let at_interval = |t: f64| current_time % t < time.delta_seconds_f64();

        if !spew.played_sound {
            if let Some(instance) = audio_assets.get_mut(&ticktock.0) {
                instance.resume(AudioTween::default());
            }
            spew.played_sound = true;
        }

        if at_interval(spew.rate_interval) {
            commands
                .spawn(ClockBundle::default())
                .set_parent(entity)
                .insert(Transform {
                    translation: spew.source_position.extend(0.0),
                    ..default()
                })
                .insert(Velocity {
                    linvel: spew.velocity,
                    ..default()
                })
                .insert(SpriteSheetBundle {
                    texture_atlas: clock.clock_atlas.clone(),
                    sprite: TextureAtlasSprite {
                        index: random::<usize>() % 6,
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 0.5),
                        ..default()
                    },
                    ..default()
                })
                .insert(Name::new("clock"));
        }
    }
}

pub fn cleanup(clocks: Query<Entity, With<Clock>>, mut commands: Commands) {
    for clock in &clocks {
        commands.entity(clock).despawn();
    }
}
