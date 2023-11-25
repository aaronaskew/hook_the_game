use super::*;
use crate::player::Player;

use rand::Rng;

#[derive(Component, Default, Copy, Clone, Reflect, PartialEq, Eq)]
pub enum EnemyState {
    /// Search for player
    #[default]
    Patrol,
    /// Move towards player until in range to attack
    Pursue,
    /// Jump at the player
    LungeAttack,
    /// Spew lots of clocks at the player
    SpewAttack,
}

/// This checks to see if the player is within the `patrol_range` of the enemy.
/// - If so, it sets the `EnemyState` to `Pursue`.
/// - If not, it sets the `EnemyState` to `Patrol`.
pub fn patrol_pursue_state_system(
    mut query: Query<(&Position, &mut EnemyState, &mut Enemy), Without<Player>>,
    player: Query<&Position, With<Player>>,
) {
    let player_position = player.single();

    for (position, mut state, mut enemy) in query.iter_mut() {
        let distance = player_position.distance(position.0);

        match (*state, distance) {
            (EnemyState::Patrol, d) if d <= enemy.patrol_range => {
                enemy.target = Some(*player_position);
                *state = EnemyState::Pursue;
            }
            (EnemyState::Pursue, d) if d > enemy.patrol_range => {
                enemy.target = None;
                *state = EnemyState::Patrol;
            }
            _ => {} // do nothing if not in range.
        }
    }
}

/// This checks to see if the player is within attack range of the enemies. If so,
/// it sets the `EnemyState` to a random attack state using `get_random_attack_state()`.
pub fn attack_state_system(
    mut query: Query<(&Position, &mut EnemyState, &mut Enemy), Without<Player>>,
    player: Query<&Position, With<Player>>,
) {
    let player_position = player.single();

    for (position, mut state, mut enemy) in query.iter_mut() {
        let distance = player_position.distance(position.0);

        match (*state, distance) {
            (EnemyState::Patrol | EnemyState::Pursue, d) if d < enemy.attack_range => {
                enemy.target = Some(*player_position);
                *state = get_random_attack_state();
            }
            _ => {} // do nothing if not in range.
        }
    }
}

/// A weighted-random attack state generator
fn get_random_attack_state() -> EnemyState {
    let mut rng = rand::thread_rng();
    let roll = rng.gen_range(0..100);

    if roll < 60 {
        EnemyState::LungeAttack
    } else if roll < 100 {
        EnemyState::SpewAttack
    } else {
        EnemyState::Patrol
    }
}
