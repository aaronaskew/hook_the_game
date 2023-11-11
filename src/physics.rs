use std::collections::HashMap;

use crate::{
    level::Ground,
    player::{self, Player},
    GameState,
};
use bevy::{prelude::*, utils::HashSet};
use bevy_rapier2d::prelude::*;

pub const NEXT_STATE: GameState = GameState::Playing;

pub struct PhysicsPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0))
            .insert_resource(RapierConfiguration {
                gravity: Vec2::new(0.0, -9.8 * 16.0),
                ..default()
            })
            .register_type::<HashSet<Entity>>()
            .add_systems(
                Update,
                next_state_after_physics_settle.run_if(in_state(GameState::InitializingPhysics)),
            );
    }
}

pub fn check_if_grounded(
    player_colliding_entities: Query<&CollidingEntities, With<Player>>,
    player_position: Query<&GlobalTransform, With<Player>>,
    ground_entities: Query<(Entity, &Collider), With<Ground>>,
    rapier_context: Res<RapierContext>,
) -> bool {
    let mut grounds: HashMap<Entity, &Collider> = HashMap::new();
    for (entity, collider) in ground_entities.iter() {
        grounds.insert(entity, collider);
    }

    let player_position = player_position.single().translation();

    if let Some(colliding_entities) = player_colliding_entities.iter().next() {
        for colliding_entity in colliding_entities.iter() {
            if grounds.contains_key(&colliding_entity) {
                // collision with ground. ensure that we are actually above it

                let ray_pos = player_position.truncate();
                let ray_dir = Vec2::new(0.0, -1.0);
                let max_toi = 4.0;
                let solid = true;
                let filter = QueryFilter::default();

                if rapier_context
                    .cast_ray(ray_pos, ray_dir, max_toi, solid, filter)
                    .is_some()
                {
                    return true;
                }
            }
        }
    }
    // otherwise, not grounded

    false
}

/// Waits until all RigidBodies are sleeping before transitioning to the next state
pub fn next_state_after_physics_settle(
    mut state: ResMut<NextState<GameState>>,
    not_sleeping_q: Query<Entity, (With<RigidBody>, With<Sleeping>)>,
) {
    if not_sleeping_q.iter().len() == 0 {
        state.set(NEXT_STATE);
    }
}

/// These are component bundles for physics-related components
pub mod bundles {
    use super::{collision_groups::*, *};

    #[derive(Clone, Debug, Bundle)]
    pub struct PlayerPhysicsBundle {
        pub collider: Collider,
        pub rigid_body: RigidBody,
        pub collision_groups: CollisionGroups,
        pub velocity: Velocity,
        pub locked_axes: LockedAxes,
        friction: Friction,
        restitution: Restitution,
    }

    impl Default for PlayerPhysicsBundle {
        fn default() -> Self {
            Self {
                collider: Collider::cuboid(
                    player::PLAYER_COLLISION_SIZE.x / 2.0,
                    player::PLAYER_COLLISION_SIZE.y / 2.0,
                ),
                rigid_body: RigidBody::Dynamic,
                collision_groups: CollisionGroups::new(PLAYER, ENEMY | GROUND | WALL | PROJECTILE),
                velocity: Velocity::zero(),
                locked_axes: LockedAxes::ROTATION_LOCKED,
                friction: Friction::new(0.0),
                restitution: Restitution::new(0.0),
            }
        }
    }

    #[derive(Clone, Debug, Bundle)]
    pub struct EnemyPhysicsBundle {
        pub collider: Collider,
        pub rigid_body: RigidBody,
        pub collision_groups: CollisionGroups,
        pub velocity: Velocity,
        pub locked_axes: LockedAxes,
        friction: Friction,
        restitution: Restitution,
    }

    impl Default for EnemyPhysicsBundle {
        fn default() -> Self {
            Self {
                collider: Collider::cuboid(32.0, 12.5),
                rigid_body: RigidBody::Dynamic,
                collision_groups: CollisionGroups::new(ENEMY, PLAYER | GROUND | WALL),
                velocity: Velocity::zero(),
                locked_axes: LockedAxes::ROTATION_LOCKED,
                friction: Friction::new(0.0),
                restitution: Restitution::new(0.0),
            }
        }
    }

    #[derive(Clone, Debug, Bundle)]
    pub struct ClockPhysicsBundle {
        pub rigid_body: RigidBody,
        pub collider: Collider,
        pub collision_groups: CollisionGroups,
        pub velocity: Velocity,
    }

    impl Default for ClockPhysicsBundle {
        fn default() -> Self {
            Self {
                rigid_body: RigidBody::Dynamic,
                collider: Collider::ball(4.5),
                collision_groups: CollisionGroups::new(PROJECTILE, PLAYER | GROUND | WALL),
                velocity: Velocity::zero(),
            }
        }
    }

    #[derive(Clone, Debug, Bundle)]
    pub struct FixedTilePhysicsBundle {
        pub collider: Collider,
        pub rigid_body: RigidBody,
        pub restitution: Restitution,
    }

    impl Default for FixedTilePhysicsBundle {
        fn default() -> Self {
            Self {
                collider: Collider::cuboid(16., 16.),
                rigid_body: RigidBody::Fixed,
                restitution: Restitution {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Average,
                },
            }
        }
    }

    #[derive(Clone, Debug, Bundle)]
    pub struct GroundPhysicsBundle {
        pub tile_physics: FixedTilePhysicsBundle,
        pub collision_groups: CollisionGroups,
        friction: Friction,
    }

    impl Default for GroundPhysicsBundle {
        fn default() -> Self {
            Self {
                collision_groups: CollisionGroups::new(GROUND, PLAYER | ENEMY | PROJECTILE),
                tile_physics: FixedTilePhysicsBundle::default(),
                friction: Friction {
                    coefficient: 13.0,
                    combine_rule: CoefficientCombineRule::Average,
                },
            }
        }
    }

    #[derive(Clone, Debug, Bundle)]
    pub struct WallPhysicsBundle {
        pub tile_physics: FixedTilePhysicsBundle,
        pub collision_groups: CollisionGroups,
    }

    impl Default for WallPhysicsBundle {
        fn default() -> Self {
            Self {
                collision_groups: CollisionGroups::new(GROUND, PLAYER | ENEMY | PROJECTILE),
                tile_physics: FixedTilePhysicsBundle::default(),
            }
        }
    }
}

mod collision_groups {
    use super::*;

    pub const PLAYER: Group = Group::GROUP_1;
    pub const WALL: Group = Group::GROUP_2;
    pub const GROUND: Group = Group::GROUP_3;
    pub const PROJECTILE: Group = Group::GROUP_4;
    pub const ENEMY: Group = Group::GROUP_5;
}
