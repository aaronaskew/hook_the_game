use std::default;

use crate::{
    enemy::Enemy,
    level::{Ground, Wall},
    player::{self, Player},
    GameState,
};
use bevy::{prelude::*, render::primitives::Aabb};
use bevy_debug_text_overlay::screen_print;
//use bevy_debug_text_overlay::screen_print;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use bevy_xpbd_2d::{parry::shape::Cuboid, prelude::*};

pub const NEXT_STATE: GameState = GameState::Playing;

// character is 32px tall, assume 2m in height, 1m = 16px
// pub const GRAVITY: f32 = 9.8 * 16.0;
pub const GRAVITY: f32 = 700.0;

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

#[derive(PhysicsLayer)]
pub enum PhysicsLayers {
    Player,
    Enemy,
    Wall,
    Ground,
}

pub struct PhysicsPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_xpbd_2d::prelude::PhysicsPlugins::default())
            .insert_resource(Gravity(Vec2::NEG_Y * GRAVITY))
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
                    InitSpriteRigidBody::Kinematic => RigidBody::Kinematic,
                    InitSpriteRigidBody::Static => RigidBody::Static,
                },
                Collider::compound(vec![(
                    Position::default(),
                    Rotation::default(),
                    Collider::cuboid(
                        player::PLAYER_COLLISION_SIZE.x,
                        player::PLAYER_COLLISION_SIZE.y,
                    ),
                )]),
                CollisionLayers::new(
                    [PhysicsLayers::Player],
                    [
                        PhysicsLayers::Enemy,
                        PhysicsLayers::Ground,
                        PhysicsLayers::Wall,
                    ],
                ),
                LockedAxes::ROTATION_LOCKED,
                Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
                ExternalForce::ZERO,
                Friction {
                    dynamic_coefficient: 0.0,
                    static_coefficient: 0.0,
                    combine_rule: CoefficientCombine::Average,
                },
                RayCaster::new(Vec2::ZERO, Vec2::NEG_Y),
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
                    InitSpriteRigidBody::Kinematic => RigidBody::Kinematic,
                    InitSpriteRigidBody::Static => RigidBody::Static,
                },
                Collider::compound(vec![(
                    Position(Vec2::new(0.0, -4.0)),
                    Rotation::default(),
                    Collider::cuboid(64.0, 25.0),
                )]),
                CollisionLayers::new(
                    [PhysicsLayers::Enemy],
                    [
                        PhysicsLayers::Player,
                        PhysicsLayers::Ground,
                        PhysicsLayers::Wall,
                    ],
                ),
                LockedAxes::ROTATION_LOCKED,
                Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
                ExternalForce::ZERO,
                Friction {
                    dynamic_coefficient: 0.0,
                    static_coefficient: 0.0,
                    combine_rule: CoefficientCombine::Average,
                },
                RayCaster::new(Vec2::ZERO, Vec2::NEG_Y),
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

        let collision_layers = match (ground, wall) {
            (Some(_), _) => CollisionLayers::new(
                [PhysicsLayers::Ground],
                [PhysicsLayers::Player, PhysicsLayers::Enemy],
            ),
            (_, Some(_)) => CollisionLayers::new(
                [PhysicsLayers::Wall],
                [PhysicsLayers::Player, PhysicsLayers::Enemy],
            ),
            _ => CollisionLayers::new(
                [PhysicsLayers::Ground],
                [PhysicsLayers::Player, PhysicsLayers::Enemy],
            ),
        };

        commands
            .entity(e)
            .insert((
                match srb {
                    InitSpriteRigidBody::Dynamic => RigidBody::Dynamic,
                    InitSpriteRigidBody::Kinematic => RigidBody::Kinematic,
                    InitSpriteRigidBody::Static => RigidBody::Static,
                },
                collider,
                collision_layers,
                LockedAxes::ROTATION_LOCKED,
                Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
                ExternalForce::ZERO,
            ))
            .remove::<InitSpriteRigidBody>();
    }
}

pub fn check_if_grounded(
    player: &Query<(&RayHits, &CollidingEntities), With<Player>>,
    grounds: &Query<Entity, With<Ground>>,
) -> bool {
    let grounds = grounds.iter().collect::<Vec<Entity>>();
    //screen_print!("check_if_ground grounds: {:#?}", grounds);
    let (hits, coll_ents) = player.single();
    // screen_print!(
    //     "check_if_ground hits: {:#?}",
    //     hits.iter().collect::<Vec<&RayHitData>>()
    // );

    for collision_entity in coll_ents.iter() {
        if grounds.contains(collision_entity) {
            // collision with ground. ensure that we are actually above it
            for hit in hits.iter() {
                if *collision_entity == hit.entity {
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
