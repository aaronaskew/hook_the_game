use crate::loading::PlayerWalkTextureAtlasAsset;
use crate::{actions::Actions, level::Ground, physics::*};
// use crate::video;
use crate::GameState;
use crate::*;
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::{CollidingEntities, LinearVelocity, Position, RayHits};

const PLAYER_SIZE: Vec2 = Vec2 { x: 10.0, y: 32.0 };

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    pub size: Vec2,
    pub is_jumping: bool,
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SpawningPlayer), spawn_player)
            .add_systems(OnExit(GameState::Playing), despawn_player)
            .add_systems(
                Update,
                (move_player, update_player_animation).run_if(in_state(GameState::Playing)),
            );
    }
}

pub fn spawn_player(
    mut commands: Commands,
    player_walk: Res<PlayerWalkTextureAtlasAsset>,
    mut state: ResMut<NextState<GameState>>,
) {
    console_log!("spawn_player start");

    let sprite = TextureAtlasSprite {
        //custom_size: Some(Vec2::splat(140.)),
        ..default()
    };

    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: player_walk.walking.clone(),
                sprite,
                transform: Transform {
                    translation: Vec3 {
                        z: 10.,
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
            AnimationTimer {
                timer: Timer::from_seconds(0.125, TimerMode::Repeating),
                frame_count: 3,
            },
        ))
        .insert(Player {
            size: PLAYER_SIZE,
            is_jumping: false,
        })
        .insert(Name::new("player"))
        .insert(physics::InitSpriteRigidBody::Dynamic);

    // After spawning the player, we need to setup the physics
    state.set(GameState::InitializingPhysics);

    console_log!("spawn_player end");
}

fn move_player(
    actions: Res<Actions>,
    physics_constants: Res<PhysicsConstants>,
    mut state: ResMut<NextState<GameState>>,
    mut player_velocity: Query<(&mut LinearVelocity, &mut Player, &Position)>,
    player_collisions_query: Query<(&RayHits, &CollidingEntities), With<Player>>,
    grounds_query: Query<Entity, With<Ground>>,
) {
    let (mut velocity, mut player, position) = player_velocity.single_mut();

    // handle moving
    if actions.player_movement.is_some() {
        let movement = Vec2::new(
            actions.player_movement.unwrap().x * physics_constants.walk_speed, // * time.delta_seconds(),
            actions.player_movement.unwrap().y * physics_constants.walk_speed, // * time.delta_seconds(),
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
        velocity.y = physics_constants.jump_speed;
    }

    // screen_print!("is_grounded: {}", is_grounded);
    // screen_print!("is_jumping: {}", player.is_jumping);
    // screen_print!("is_grounded: {}", is_grounded);

    //check for wall collisions and thus death
    if position.x.abs() > 400. || position.y.abs() > 300. {
        state.set(GameState::PlayingCutScene);
    }
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

fn despawn_player(players: Query<(Entity, With<Player>)>, mut commands: Commands) {
    for (player, _) in &players {
        commands.entity(player).despawn();
    }
}
