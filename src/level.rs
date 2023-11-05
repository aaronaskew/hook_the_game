use crate::{loading::LevelAsset, GameState};
use bevy::prelude::*;
//use bevy_debug_text_overlay::screen_print;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component)]
pub struct Ground;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .register_type::<EntityInstance>()
            .insert_resource(LevelSelection::Index(0))
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                set_clear_color: SetClearColor::FromLevelBackground,
                int_grid_rendering: IntGridRendering::Invisible,
                level_background: LevelBackground::Nonexistent,
            })
            .add_systems(OnEnter(GameState::LoadingLevel), setup)
            .add_systems(OnExit(GameState::Playing), cleanup);
    }
}

fn setup(mut commands: Commands, level: Res<LevelAsset>, mut state: ResMut<NextState<GameState>>) {
    //screen_print!("level setup");

    commands.spawn((
        LdtkWorldBundle {
            ldtk_handle: level.level.clone(),

            ..default()
        },
        Name::new("level"),
    ));

    state.set(GameState::SpawningEntities);
}

fn cleanup(levels: Query<Entity, With<LevelSet>>, mut commands: Commands) {
    for (level) in &levels {
        commands.entity(level).despawn();
    }
}
