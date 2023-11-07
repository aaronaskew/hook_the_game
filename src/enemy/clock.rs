use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use rand::random;

use crate::{loading::ClockTextureAtlasAsset, physics::PhysicsLayers, player::Player};

#[derive(Component)]
pub struct Clock {
    pub lifetime: f32,
}

#[derive(Bundle)]
pub struct ClockBundle {
    pub clock: Clock,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub collision_layers: CollisionLayers,
}

impl Default for ClockBundle {
    fn default() -> Self {
        Self {
            clock: Clock { lifetime: 5.0 },
            rigid_body: RigidBody::Dynamic,
            collider: Collider::ball(9.0),
            collision_layers: CollisionLayers::new(
                [PhysicsLayers::Projectile],
                [
                    PhysicsLayers::Player,
                    PhysicsLayers::Ground,
                    PhysicsLayers::Wall,
                ],
            ),
        }
    }
}

#[derive(Component, Clone, Reflect)]
pub struct SpewClocks {
    pub source_position: Vec2,
    pub velocity: Vec2,
    pub rate_interval: f64,
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
            if entity == &player_entity {
                player.is_alive = false;
            }
        }
    }
}

pub fn spew_clocks(
    mut commands: Commands,
    query: Query<&SpewClocks>,
    time: Res<Time>,
    clock: Res<ClockTextureAtlasAsset>,
) {
    for spew in query.iter() {
        let current_time = time.elapsed_seconds_f64();
        let at_interval = |t: f64| current_time % t < time.delta_seconds_f64();

        if at_interval(spew.rate_interval) {
            commands
                .spawn(ClockBundle::default())
                .insert(Position(spew.source_position))
                .insert(LinearVelocity(spew.velocity))
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
