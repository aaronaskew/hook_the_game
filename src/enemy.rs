use crate::loading::EnemyWalkTextureAtlasAsset;
use crate::GameState;
use crate::*;
use crate::{actions::Actions, level::Ground, physics::*};
use bevy_ecs_ldtk::prelude::*;
use bevy_xpbd_2d::prelude::{CollidingEntities, LinearVelocity, Position, RayHits};

const ENEMY_COLLISION_SIZE: Vec2 = Vec2 { x: 64.0, y: 32.0 };

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // register the Enemy type to see the details in the egui inspector
            .register_type::<Enemy>()
            // register the EnemyLdtkBundle in order to spawn the enemy entity via
            // the ldtk level
            .register_ldtk_entity::<EnemyLdtkBundle>("Enemy")
            .add_systems(OnEnter(GameState::SpawningEntities), initialize_enemies)
            .add_systems(
                Update,
                (move_enemy, update_enemy_animation, death_check)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), cleanup);
    }
}

#[derive(Component, Reflect)]
pub struct Enemy {
    pub is_walking: bool,
    pub is_jumping: bool,
    pub is_alive: bool,
}

// implement default()
impl Default for Enemy {
    fn default() -> Self {
        Enemy {
            is_walking: false,
            is_jumping: false,
            is_alive: true,
        }
    }
}

/// this is the bundle that will be instanced when the enemy entity is loaded from
/// the ldtk level. further initialization will be done by the system `initialize_enemy`
#[derive(Default, Bundle, LdtkEntity)]
pub struct EnemyLdtkBundle {
    enemy: Enemy,
}

/// This is the system that will be called after the enemy is
/// instanced from the ldtk level. The majority of the initialization
/// takes place here.
fn initialize_enemies(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Enemy>>,
    enemy_walk: Res<EnemyWalkTextureAtlasAsset>,
    mut state: ResMut<NextState<GameState>>,
) {
    for (entity, transform) in query.iter() {
        commands
            .entity(entity)
            .insert((
                SpriteSheetBundle {
                    texture_atlas: enemy_walk.walking.clone(),
                    sprite: TextureAtlasSprite::default(),
                    transform: *transform,
                    ..default()
                },
                AnimationTimer {
                    timer: Timer::from_seconds(0.125, TimerMode::Repeating),
                    frame_count: 2,
                },
            ))
            .insert(Name::new("enemy"))
            .insert(physics::InitSpriteRigidBody::Dynamic);
    }
}

fn move_enemy(
    mut enemy_velocity: Query<(&mut LinearVelocity, &mut Enemy, &Position)>,
    enemy_collisions_query: Query<(&RayHits, &CollidingEntities), With<Enemy>>,
    grounds_query: Query<Entity, With<Ground>>,
) {
    for (mut velocity, mut enemy, position) in enemy_velocity.iter_mut() {

        // TODO enemy ai

        // // handle moving
        // if actions.enemy_movement.is_some() {
        //     let movement = Vec2::new(
        //         actions.enemy_movement.unwrap().x * physics_constants.walk_speed, // * time.delta_seconds(),
        //         actions.enemy_movement.unwrap().y * physics_constants.walk_speed, // * time.delta_seconds(),
        //     );

        //     velocity.x = movement.x;
        // }

        // // handle jumping
        // let is_grounded = physics::check_if_grounded(&enemy_collisions_query, &grounds_query);
        // if enemy.is_jumping {
        //     if is_grounded {
        //         enemy.is_jumping = false;
        //     }
        // } else if actions.jump && is_grounded {
        //     enemy.is_jumping = true;
        //     velocity.y = physics_constants.jump_speed;
        // }

        // screen_print!("is_grounded: {}", is_grounded);
        // screen_print!("is_jumping: {}", enemy.is_jumping);
        // screen_print!("is_grounded: {}", is_grounded);
    }
}

fn update_enemy_animation(
    mut sprites: Query<(&mut TextureAtlasSprite, &mut AnimationTimer, &Enemy)>,
    time: Res<Time>,
    actions: Res<Actions>,
) {
    for (mut sprite, mut animation_timer, enemy) in &mut sprites {
        animation_timer.timer.tick(time.delta());

        // if actions.enemy_movement.is_none() {
        //     return;
        // }

        // match actions.enemy_movement {
        //     Some(vec2) if vec2.x > 0. => {
        //         sprite.flip_x = false;
        //     }
        //     Some(vec2) if vec2.x < 0. => {
        //         sprite.flip_x = true;
        //     }
        //     Some(_) | None => (),
        // }

        if animation_timer.timer.just_finished() && enemy.is_walking {
            sprite.index = (sprite.index + 1) % animation_timer.frame_count;
        }
    }
}

fn cleanup(enemies: Query<(Entity, With<Enemy>)>, mut commands: Commands) {
    for (enemy, _) in &enemies {
        commands.entity(enemy).despawn();
    }
}

fn death_check(
    enemy: Query<(Entity, &Enemy)>,
    mut state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    for (entity, enemy) in enemy.iter() {
        if !enemy.is_alive {
            commands.entity(entity).despawn();
        }
    }
}
