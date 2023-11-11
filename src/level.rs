use crate::{loading::LevelAsset, GameState, physics};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

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
            .register_ldtk_int_cell::<GroundBundle>(2)
            .register_ldtk_int_cell::<WallBundle>(3)
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
    for level in &levels {
        commands.entity(level).despawn();
    }
}

#[derive(Component, Clone, Debug, Default)]
pub struct Ground;

#[derive(Component, Clone, Debug, Default)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderTileBundle {
    pub name: Name,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct GroundBundle {
    pub ground: Ground,
    pub physics_bundle: physics::bundles::GroundPhysicsBundle,
    #[from_int_grid_cell]
    pub collider_tile_bundle: ColliderTileBundle,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    pub wall: Wall,
    pub physics_bundle: physics::bundles::WallPhysicsBundle,
    #[from_int_grid_cell]
    pub collider_tile_bundle: ColliderTileBundle,
}

impl From<IntGridCell> for ColliderTileBundle {
    fn from(int_grid_cell: IntGridCell) -> Self {
        match int_grid_cell.value {
            2 => ColliderTileBundle {
                name: Name::new("Ground"),
                
            },
            3 => ColliderTileBundle {
                name: Name::new("Wall"),
                
            },
            _ => ColliderTileBundle::default(),
        }
    }
}
