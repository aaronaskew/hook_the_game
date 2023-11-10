use std::collections::HashMap;

use crate::{
    enemy::Enemy,
    level::{Ground, Wall},
    player::{self, Player},
    GameState,
};
use bevy::{prelude::*, render::primitives::Aabb, utils::HashSet};
use bevy_rapier2d::prelude::*;

pub const NEXT_STATE: GameState = GameState::Playing;

// character is 32px tall, assume 2m in height, 1m = 16px
// pub const GRAVITY: f32 = 9.8 * 16.0;
// pub const GRAVITY: f32 = 700.0;

// #[derive(Reflect, Resource, Default, InspectorOptions)]
// #[reflect(Resource, InspectorOptions)]
// pub struct PhysicsConstants {
//     pub walk_speed: f32,
//     pub jump_speed: f32,
// }

/// A component that tells us to initialize the physics on this entity according
/// to the type and shape of the sprite
#[derive(Component, Clone, Debug, Default)]
#[allow(dead_code)]
pub enum InitSpriteRigidBody {
    Dynamic,
    Kinematic,
    #[default]
    Static,
}

pub const COLLISION_GROUP_PLAYER: Group = Group::GROUP_1;
pub const COLLISION_GROUP_WALL: Group = Group::GROUP_2;
pub const COLLISION_GROUP_GROUND: Group = Group::GROUP_3;
pub const COLLISION_GROUP_PROJECTILE: Group = Group::GROUP_4;
pub const COLLISION_GROUP_ENEMY: Group = Group::GROUP_5;

pub struct PhysicsPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0))
            .register_type::<HashSet<Entity>>()
            .add_systems(OnEnter(GameState::InitializingPhysics), init_sprite_physics)
            .add_systems(
                Update,
                next_state_after_physics_settle
                    .run_if(in_state(GameState::InitializingPhysics))
                    .after(init_sprite_physics),
            );
    }
}

fn collider_from_aabb(aabb: &Aabb) -> Collider {
    let extents = aabb.half_extents * 2.0;
    Collider::cuboid(extents.x, extents.y)
}
/// init physics based on sprite shape and InitSpriteRigidbody type
pub fn init_sprite_physics(
    mut commands: Commands,
    non_living: Query<
        (
            Entity,
            Option<&Aabb>,
            &InitSpriteRigidBody,
            Option<&Ground>,
            Option<&Wall>,
        ),
        (Without<Player>, Without<Enemy>),
    >,
    player: Query<(Entity, &InitSpriteRigidBody), With<Player>>,
    enemy: Query<(Entity, &InitSpriteRigidBody), With<Enemy>>,
) {
    console_log!(
        "init_sprite_physics: non-player sprites: {}",
        non_living.iter().len()
    );

    // set up player entities using the player.size for the collider
    for (e, srb) in player.iter() {
        commands
            .entity(e)
            .insert((
                match srb {
                    InitSpriteRigidBody::Dynamic => RigidBody::Dynamic,
                    InitSpriteRigidBody::Kinematic => RigidBody::KinematicPositionBased,
                    InitSpriteRigidBody::Static => RigidBody::Fixed,
                },
                Collider::cuboid(
                    player::PLAYER_COLLISION_SIZE.x / 2.0,
                    player::PLAYER_COLLISION_SIZE.y / 2.0,
                ),
                CollisionGroups::new(
                    COLLISION_GROUP_PLAYER,
                    COLLISION_GROUP_ENEMY
                        | COLLISION_GROUP_GROUND
                        | COLLISION_GROUP_WALL
                        | COLLISION_GROUP_PROJECTILE,
                ),
                // CollisionGroups::new(
                //     [COLLISION_GROUP_Player],
                //     [
                //         COLLISION_GROUP_Enemy,
                //         COLLISION_GROUP_Ground,
                //         COLLISION_GROUP_Wall,
                //         COLLISION_GROUP_Projectile,
                //     ],
                // ),
                LockedAxes::ROTATION_LOCKED,
                Restitution {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
                Friction {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Average,
                },
            ))
            .remove::<InitSpriteRigidBody>();
    }

    // set up enemy entities
    for (e, srb) in enemy.iter() {
        commands
            .entity(e)
            .insert((
                match srb {
                    InitSpriteRigidBody::Dynamic => RigidBody::Dynamic,
                    InitSpriteRigidBody::Kinematic => RigidBody::KinematicPositionBased,
                    InitSpriteRigidBody::Static => RigidBody::Fixed,
                },
                Collider::compound(vec![(
                    Vec2::new(0.0, -4.0),
                    Rot::default(),
                    Collider::cuboid(64.0, 25.0),
                )]),
                CollisionGroups::new(
                    COLLISION_GROUP_ENEMY,
                    COLLISION_GROUP_PLAYER | COLLISION_GROUP_GROUND | COLLISION_GROUP_WALL,
                ),
                LockedAxes::ROTATION_LOCKED,
                Restitution {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
                Friction {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Average,
                },
            ))
            .remove::<InitSpriteRigidBody>();
    }

    // set up non-player entities, using the Aabb bounds of the sprite for the collider
    for (e, aabb, srb, ground, wall) in non_living.iter() {
        let collider;

        if let Some(aabb) = aabb {
            collider = collider_from_aabb(aabb);
        } else {
            //if there is no Aabb component, assuming these are 32x32 sprites
            collider = Collider::cuboid(32., 32.);
        }

        // let friction = if ground.is_some() {
        //     Friction {
        //         dynamic_coefficient: 0.1,
        //         static_coefficient: 0.0,
        //         combine_rule: CoefficientCombine::Average,
        //     }
        // } else if wall.is_some() {
        //     Friction {
        //         dynamic_coefficient: 0.0,
        //         static_coefficient: 0.0,
        //         combine_rule: CoefficientCombine::Average,
        //     }
        // } else {
        //     Friction::default()
        // };

        let collision_groups = match (ground, wall) {
            (Some(_), _) => CollisionGroups::new(
                COLLISION_GROUP_GROUND,
                COLLISION_GROUP_PLAYER | COLLISION_GROUP_ENEMY | COLLISION_GROUP_PROJECTILE,
            ),
            (_, Some(_)) => CollisionGroups::new(
                COLLISION_GROUP_WALL,
                COLLISION_GROUP_PLAYER | COLLISION_GROUP_ENEMY | COLLISION_GROUP_PROJECTILE,
            ),
            _ => CollisionGroups::new(
                COLLISION_GROUP_GROUND,
                COLLISION_GROUP_PLAYER | COLLISION_GROUP_ENEMY | COLLISION_GROUP_PROJECTILE,
            ),
        };

        commands
            .entity(e)
            .insert((
                match srb {
                    InitSpriteRigidBody::Dynamic => RigidBody::Dynamic,
                    InitSpriteRigidBody::Kinematic => RigidBody::KinematicPositionBased,
                    InitSpriteRigidBody::Static => RigidBody::Fixed,
                },
                collider,
                collision_groups,
                LockedAxes::ROTATION_LOCKED,
                Restitution {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
            ))
            .remove::<InitSpriteRigidBody>();
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

    for colliding_entity in player_colliding_entities.single().iter() {
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
