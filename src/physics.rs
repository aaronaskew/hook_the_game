use bevy::{prelude::*, render::primitives::Aabb};
use bevy_rapier2d::{prelude::*, rapier::prelude::RigidBodyBuilder};

use crate::{player::*, GameState};

const PIXELS_PER_METER: f32 = 100.;

pub struct PhysicsPlugin;

/// This plugin is responsible for the playing video.
/// The video is only played during the State `GameState::CutScene` and is removed when that state is exited
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(OnEnter(GameState::InitializingPhysics), setup_physics);
        //.add_system(print_ball_altitude);
    }
}

pub fn setup_physics(
    mut commands: Commands,
    query: Query<(Entity, &Aabb), With<Player>>,
    mut state: ResMut<NextState<GameState>>,
) {
    console_log!("setup_physics start");

    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));

    // Setup Player Physics
    let (entity, aabb) = query.single();
    commands
        .entity(entity)
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ExternalForce::default())
        .insert(ExternalImpulse::default())
        .insert(Collider::cuboid(
            aabb.half_extents.x * 0.3,
            aabb.half_extents.y,
        ))
        .insert(GravityScale(1.0))
        .insert(Damping {
            linear_damping: 5.0,
            angular_damping: 0.0,
        });

    // let (entity, aabb) = player.single();
    // commands
    //     .entity(entity)
    //     .insert(RigidBody::Dynamic)
    //     .insert(Collider::cuboid(aabb.half_extents.x, aabb.half_extents.y));

    // After initializing the physics, we can start the game
    state.set(GameState::Playing);

    console_log!("setup_physics end");
}
