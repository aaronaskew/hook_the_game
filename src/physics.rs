use bevy::{prelude::*, render::primitives::Aabb};
use bevy_debug_text_overlay::screen_print;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use bevy_xpbd_2d::prelude::*;

use crate::{level::Ground, player::Player, GameState};

// character is 32px tall
// assume 2m in height
// 1m = 16px
pub const GRAVITY: f32 = 9.8 * 16.0;
pub const WALK_SPEED: f32 = 150.;
pub const JUMP_SPEED: f32 = 500.;

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct PhysicsConstants {
    pub walk_speed: f32,
    pub jump_speed: f32,
}

/// A component that tells us to initialize the physics on this entity according
/// to the type and shape of the sprite
#[derive(Component)]
#[allow(dead_code)]
pub enum InitSpriteRigidBody {
    Dynamic,
    Kinematic,
    Static,
}

pub struct PhysicsPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default())
            .insert_resource(Gravity(Vec2::NEG_Y * GRAVITY))
            .add_systems(OnEnter(GameState::InitializingPhysics), init_sprite_physics)
            .add_systems(OnEnter(GameState::Playing), debug_ground)
            .insert_resource(PhysicsConstants {
                walk_speed: WALK_SPEED,
                jump_speed: JUMP_SPEED,
            })
            .register_type::<PhysicsConstants>();
    }
}

fn collider_from_aabb(aabb: &Aabb) -> Collider {
    let extents = aabb.half_extents * 2.0;
    Collider::cuboid(extents.x, extents.y)
}
/// init physics based on sprite shape and InitSpriteRigidbody type
pub fn init_sprite_physics(
    mut commands: Commands,
    query: Query<(Entity, &Aabb, &InitSpriteRigidBody), Without<Player>>,
    player: Query<(Entity, &InitSpriteRigidBody, &Player)>,
    mut state: ResMut<NextState<GameState>>,
) {
    // set up player entities using the player.size for the collider
    for (e, srb, player) in player.iter() {
        commands
            .entity(e)
            .insert((
                match srb {
                    InitSpriteRigidBody::Dynamic => RigidBody::Dynamic,
                    InitSpriteRigidBody::Kinematic => RigidBody::Kinematic,
                    InitSpriteRigidBody::Static => RigidBody::Static,
                },
                Collider::cuboid(player.size.x, player.size.y),
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
    for (e, aabb, srb) in query.iter() {
        commands
            .entity(e)
            .insert((
                match srb {
                    InitSpriteRigidBody::Dynamic => RigidBody::Dynamic,
                    InitSpriteRigidBody::Kinematic => RigidBody::Kinematic,
                    InitSpriteRigidBody::Static => RigidBody::Static,
                },
                collider_from_aabb(aabb),
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

    state.set(GameState::Playing);
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

// TODO testing physics

pub fn debug_ground(mut commands: Commands) {
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(700.0, 10.),
        //Name::new("ground"),
        Position(Vec2::NEG_Y * 250.0),
        Ground,
    ));
}
