use bevy::prelude::*;

use super::*;

pub fn update_enemy_animation(
    mut sprites: Query<(&mut TextureAtlasSprite, &mut AnimationSettings, &Enemy)>,
    time: Res<Time>,
) {
    for (mut sprite, mut animation_settings, enemy) in &mut sprites {
        let mut timer = &mut animation_settings.animation_timer;

        timer.timer.tick(time.delta());

        if timer.timer.just_finished() {
            let new_index = (sprite.index + 1) % animation_settings.frames.len();

            if !animation_settings.frames.contains(&new_index) {
                sprite.index = animation_settings.frames[0];
            } else {
                sprite.index = new_index;
            }
        }

        // handle sprite flipping
        sprite.flip_x = !enemy.facing_left;
    }
}

#[derive(Component, Clone, Reflect)]
pub struct AnimationSettings {
    pub frames: Vec<usize>,
    pub animation_timer: AnimationTimer,
    pub last_state: EnemyState,
}

/// This is a system to check the Enemy State and update the animation depending on
/// the state.
pub fn animation_controller(
    mut query: Query<(&mut AnimationSettings, &EnemyState, &mut TextureAtlasSprite)>,
) {
    for (mut anim_settings, state, mut sprite) in &mut query {
        if *state == anim_settings.last_state {
            continue;
        } else {
            sprite.index = anim_settings.frames[0];
        }

        match state {
            EnemyState::Patrol => {
                *anim_settings = AnimationSettings {
                    frames: vec![0, 1],
                    animation_timer: AnimationTimer {
                        timer: Timer::from_seconds(0.125, TimerMode::Repeating),
                        frame_count: 2,
                    },
                    last_state: EnemyState::Patrol,
                }
            }
            EnemyState::Pursue => {
                *anim_settings = AnimationSettings {
                    frames: vec![0, 1, 2],
                    animation_timer: AnimationTimer {
                        timer: Timer::from_seconds(0.125, TimerMode::Repeating),
                        frame_count: 3,
                    },
                    last_state: EnemyState::Pursue,
                }
            }
            EnemyState::LungeAttack => {
                *anim_settings = AnimationSettings {
                    frames: vec![2],
                    animation_timer: AnimationTimer {
                        timer: Timer::from_seconds(0.125, TimerMode::Repeating),
                        frame_count: 1,
                    },
                    last_state: EnemyState::LungeAttack,
                }
            }
            EnemyState::SpewAttack => {
                *anim_settings = AnimationSettings {
                    frames: vec![3],
                    animation_timer: AnimationTimer {
                        timer: Timer::from_seconds(0.125, TimerMode::Repeating),
                        frame_count: 1,
                    },
                    last_state: EnemyState::SpewAttack,
                }
            }
        }
    }
}

/// This system sets the enemy actions and processes them based on the current state
pub fn process_actions(
    mut query: Query<(&mut LinearVelocity, &mut EnemyState, &mut Enemy, &Position)>,
    time: Res<Time>,
) {
    for (mut velocity, mut state, mut enemy, position) in query.iter_mut() {
        let mut target_delta;

        if let Some(target) = enemy.target {
            target_delta = target.0 - position.0;
        } else {
            target_delta = Vec2::ZERO;
        }

        // set current action if next_state is present
        if let Some(next_action) = &enemy.next_action {
            enemy.current_action = next_action.clone();
            enemy.next_action = None;
        }

        let is_grounded = enemy.is_grounded;

        // set velocity
        match enemy.current_action {
            EnemyAction::Patrol {
                ref mut direction_timer,
                speed,
            } => {
                direction_timer.tick(time.delta());
                if direction_timer.just_finished() {
                    enemy.facing_left = !enemy.facing_left;
                }
                velocity.x = if enemy.facing_left { -speed } else { speed };
            }
            EnemyAction::Pursue { speed } => {
                velocity.x = if target_delta.x > 0.0 { speed } else { -speed };
            }
            EnemyAction::LungeAttack {
                ref mut before_lunge_timer,
                ref mut after_lunge_timer,
                speed,
            } => {
                before_lunge_timer.tick(time.delta());

                if before_lunge_timer.just_finished() {
                    //lunge
                    velocity.x = if enemy.facing_left { -speed } else { speed };
                    velocity.y = speed;
                } else if before_lunge_timer.finished() && is_grounded {
                    after_lunge_timer.tick(time.delta());

                    if after_lunge_timer.just_finished() {
                        //lunge action done, go to patrol state
                        *state = EnemyState::Patrol;
                    }
                }
            }
            EnemyAction::SpewAttack {
                ref mut spew_timer,
                spew_rate,
                spew_min_velocity,
                spew_max_velocity,
                min_angle,
                max_angle,
            } => {
                spew_timer.tick(time.delta());
                if spew_timer.just_finished() {
                    *state = EnemyState::Patrol;
                } else {
                    //spew clocks
                }
            }
        }

        // set direction
        match (*state, velocity.x, target_delta) {
            (_, vx, _) if vx > 0.0 => {
                enemy.facing_left = false;
            }
            (_, vx, _) if vx < 0.0 => {
                enemy.facing_left = true;
            }
            (EnemyState::LungeAttack | EnemyState::SpewAttack, vx, target_delta)
                if vx == 0.0 && target_delta.x > 0.0 =>
            {
                enemy.facing_left = false;
            }
            (EnemyState::LungeAttack | EnemyState::SpewAttack, vx, target_delta)
                if vx == 0.0 && target_delta.x < 0.0 =>
            {
                enemy.facing_left = true;
            }
            _ => (),
        }
    }
}

pub fn process_state_change(mut query: Query<(&mut Enemy, &EnemyState), Changed<EnemyState>>) {
    for (mut enemy, state) in query.iter_mut() {
        enemy.next_action = match state {
            EnemyState::Patrol => Some(EnemyAction::Patrol {
                direction_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
                speed: 120.0,
            }),
            EnemyState::Pursue => Some(EnemyAction::Pursue { speed: 200.0 }),
            EnemyState::LungeAttack => Some(EnemyAction::LungeAttack {
                before_lunge_timer: Timer::from_seconds(1.0, TimerMode::Once),
                after_lunge_timer: Timer::from_seconds(2.0, TimerMode::Once),
                speed: 200.0,
            }),
            EnemyState::SpewAttack => Some(EnemyAction::SpewAttack {
                spew_timer: Timer::from_seconds(5.0, TimerMode::Once),
                spew_rate: 1.0,
                spew_min_velocity: 100.0,
                spew_max_velocity: 200.0,
                min_angle: 0.0,
                max_angle: 90.0,
            }),
        }
    }
}
