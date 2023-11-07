use crate::loading::EnemyTextureAtlasAsset;
use crate::player::Player;
use crate::GameState;
use crate::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_xpbd_2d::prelude::*;
use state::EnemyState;

use self::animation::AnimationSettings;

mod animation;
mod clock;
mod state;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // register the Enemy type to see the details in the egui inspector
            .register_type::<Enemy>()
            .register_type::<EnemyAction>()
            .register_type::<EnemyState>()
            .register_type::<AnimationSettings>()
            // register the EnemyLdtkBundle in order to spawn the enemy entity via
            // the ldtk level
            .register_ldtk_entity::<EnemyLdtkBundle>("Enemy")
            .add_systems(OnEnter(GameState::SpawningEntities), initialize_enemies)
            .add_systems(
                Update,
                (
                    animation::process_actions,
                    animation::animation_controller,
                    animation::update_enemy_animation,
                    animation::process_state_change,
                    state::attack_state_system,
                    state::patrol_pursue_state_system,
                    check_collisions_with_player,
                    clock::update_clocks,
                    clock::check_collisions_with_player,
                    clock::spew_clocks,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), (cleanup, clock::cleanup));
    }
}

#[derive(Reflect, Clone)]
pub enum EnemyAction {
    Patrol {
        direction_timer: Timer,
        speed: f32,
    },
    Pursue {
        speed: f32,
    },
    LungeAttack {
        before_lunge_timer: Timer,
        after_lunge_timer: Timer,
        speed: f32,
    },
    SpewAttack {
        spew_timer: Timer,
        spew_rate: f64,
        spew_min_velocity: f32,
        spew_max_velocity: f32,
    },
}

impl Default for EnemyAction {
    fn default() -> Self {
        Self::Patrol {
            direction_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
            speed: 120.0,
        }
    }
}

#[derive(Component, Reflect, Clone)]
pub struct Enemy {
    pub facing_left: bool,
    pub patrol_range: f32,
    pub attack_range: f32,
    pub is_grounded: bool,
    pub target: Option<Position>,
    pub current_action: EnemyAction,
    pub next_action: Option<EnemyAction>,
}

// implement default()
impl Default for Enemy {
    fn default() -> Self {
        Enemy {
            facing_left: true,
            patrol_range: 160.,
            attack_range: 80.,
            is_grounded: true,
            target: None,
            current_action: EnemyAction::default(),
            next_action: None,
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
    enemy: Res<EnemyTextureAtlasAsset>,
) {
    for (entity, transform) in query.iter() {
        commands
            .entity(entity)
            .insert((
                SpriteSheetBundle {
                    texture_atlas: enemy.enemy_atlas.clone(),
                    sprite: TextureAtlasSprite::default(),
                    transform: *transform,
                    ..default()
                },
                AnimationSettings {
                    frames: vec![0, 1],
                    animation_timer: AnimationTimer {
                        timer: Timer::from_seconds(0.125, TimerMode::Repeating),
                        frame_count: 2,
                    },
                    last_state: EnemyState::Patrol,
                },
            ))
            .insert(Name::new("enemy"))
            .insert(EnemyState::Patrol)
            .insert(physics::InitSpriteRigidBody::Dynamic);
    }
}

/// This system will check for collisions with the player. If the player
/// is hit, the player is killed.
pub fn check_collisions_with_player(
    query: Query<&CollidingEntities, &Enemy>,
    mut player: Query<(Entity, &mut Player)>,
) {
    let (player_entity, mut player) = player.single_mut();

    for colliding_entities in query.iter() {
        for entity in colliding_entities.iter() {
            if entity == &player_entity {
                player.is_alive = false;
            }
        }
    }
}

fn cleanup(enemies: Query<(Entity, With<Enemy>)>, mut commands: Commands) {
    for (enemy, _) in &enemies {
        commands.entity(enemy).despawn();
    }
}
