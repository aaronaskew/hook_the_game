use crate::loading::PlayerWalkTextureAtlasAsset;
use crate::GameState;
use crate::*;
use crate::{actions::Actions, level::Ground};
use bevy_ecs_ldtk::prelude::*;
use bevy_xpbd_2d::prelude::{CollidingEntities, LinearVelocity, RayHits};

pub const PLAYER_COLLISION_SIZE: Vec2 = Vec2 { x: 10.0, y: 32.0 };
pub const WALK_SPEED: f32 = 150.;
pub const JUMP_SPEED: f32 = 300.;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // register the Player type to see the details in the egui inspector
            .register_type::<Player>()
            // register the PlayerLdtkBundle in order to spawn the player entity via
            // the ldtk level
            .register_ldtk_entity::<PlayerLdtkBundle>("Player")
            .add_systems(OnEnter(GameState::SpawningEntities), initialize_player)
            .add_systems(
                Update,
                (move_player, update_player_animation, death_check)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), cleanup);
    }
}

#[derive(Component, Reflect)]
pub struct Player {
    pub walk_speed: f32,
    pub jump_speed: f32,
    pub is_jumping: bool,
    pub is_alive: bool,
}

// implement default()
impl Default for Player {
    fn default() -> Self {
        Player {
            walk_speed: WALK_SPEED,
            jump_speed: JUMP_SPEED,
            is_jumping: false,
            is_alive: true,
        }
    }
}

/// this is the bundle that will be instanced when the player entity is loaded from
/// the ldtk level. further initialization will be done by the system `initialize_player`
#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerLdtkBundle {
    player: Player,
    #[worldly]
    worldly: Worldly,
}

/// This is the system that will be called after the player is
/// instanced from the ldtk level. The majority of the initialization
/// takes place here.
fn initialize_player(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Player>>,
    player_walk: Res<PlayerWalkTextureAtlasAsset>,
    mut state: ResMut<NextState<GameState>>,
) {
    let (entity, transform) = query.single();

    commands
        .entity(entity)
        .insert((
            SpriteSheetBundle {
                texture_atlas: player_walk.walking.clone(),
                sprite: TextureAtlasSprite::default(),
                transform: *transform,
                ..default()
            },
            AnimationTimer {
                timer: Timer::from_seconds(0.125, TimerMode::Repeating),
                frame_count: 3,
            },
        ))
        .insert(Name::new("player"))
        .insert(physics::InitSpriteRigidBody::Dynamic);

    // After initializing the player, we need to setup the physics
    state.set(GameState::InitializingPhysics);
}

fn move_player(
    actions: Res<Actions>,
    mut player_velocity: Query<(&mut LinearVelocity, &mut Player)>,
    player_collisions_query: Query<(&RayHits, &CollidingEntities), With<Player>>,
    grounds_query: Query<Entity, With<Ground>>,
) {
    let (mut velocity, mut player) = player_velocity.single_mut();

    // handle moving
    if actions.player_movement.is_some() {
        let movement = Vec2::new(
            actions.player_movement.unwrap().x * player.walk_speed,
            actions.player_movement.unwrap().y * player.walk_speed,
        );

        velocity.x = movement.x;
    }

    // handle jumping
    let is_grounded = physics::check_if_grounded(&player_collisions_query, &grounds_query);
    if player.is_jumping {
        if is_grounded {
            player.is_jumping = false;
        }
    } else if actions.jump && is_grounded {
        player.is_jumping = true;
        velocity.y = player.jump_speed;
    }

    // screen_print!("is_grounded: {}", is_grounded);
    // screen_print!("is_jumping: {}", player.is_jumping);
    // screen_print!("is_grounded: {}", is_grounded);
}

fn update_player_animation(
    mut sprites: Query<(&mut TextureAtlasSprite, &mut AnimationTimer, With<Player>)>,
    time: Res<Time>,
    actions: Res<Actions>,
) {
    for (mut sprite, mut animation_timer, _) in &mut sprites {
        animation_timer.timer.tick(time.delta());

        if actions.player_movement.is_none() {
            return;
        }

        match actions.player_movement {
            Some(vec2) if vec2.x > 0. => {
                sprite.flip_x = false;
            }
            Some(vec2) if vec2.x < 0. => {
                sprite.flip_x = true;
            }
            Some(_) | None => (),
        }

        if animation_timer.timer.just_finished() && actions.player_movement.is_some() {
            sprite.index = (sprite.index + 1) % animation_timer.frame_count;
        }
    }
}

fn cleanup(players: Query<(Entity, With<Player>)>, mut commands: Commands) {
    for (player, _) in &players {
        commands.entity(player).despawn();
    }
}

fn death_check(player: Query<&Player>, mut state: ResMut<NextState<GameState>>) {
    if !player.single().is_alive {
        state.set(GameState::PlayingCutScene);
    }
}
