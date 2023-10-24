use crate::actions::Actions;
use crate::loading::PlayerWalk;
// use crate::video;
use crate::GameState;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    // alive: bool,
}

#[derive(Component)]
pub struct AnimationTimer {
    pub timer: Timer,
    pub frame_count: usize,
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(OnExit(GameState::Playing), despawn_player)
            .add_systems(Update, move_player.run_if(in_state(GameState::Playing)))
            .add_systems(Update, update_player_animation);
    }
}

fn spawn_player(mut commands: Commands, player_walk: Res<PlayerWalk>) {
    let sprite = TextureAtlasSprite {
        custom_size: Some(Vec2::splat(140.)),
        ..default()
    };

    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: player_walk.walking.clone(),
                sprite,
                ..default()
            },
            AnimationTimer {
                timer: Timer::from_seconds(0.125, TimerMode::Repeating),
                frame_count: 3,
            },
        ))
        .insert(Player {});
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut state: ResMut<NextState<GameState>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut player_transform in &mut player_query {
        player_transform.translation += movement;

        let x = player_transform.translation.x;
        let y = player_transform.translation.y;

        //check for wall collisions and thus death
        console_log!("x: {} y: {}", x, y);

        if x.abs() > 400. || y.abs() > 300. {
            state.set(GameState::PlayingCutScene);
            player_transform.translation = Vec3::ZERO;
        }
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
