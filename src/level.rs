use crate::{loading::LevelAsset, physics::InitSpriteRigidBody, GameState};
use bevy::prelude::*;
//use bevy_debug_text_overlay::screen_print;
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
pub struct GroundBundle {
    #[with(init_sprite_rigidbody_static)]
    pub rigid_body: InitSpriteRigidBody,
    pub ground: Ground,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    #[with(init_sprite_rigidbody_static)]
    pub rigid_body: InitSpriteRigidBody,
    pub wall: Wall,
}

pub fn init_sprite_rigidbody_static(_: IntGridCell) -> InitSpriteRigidBody {
    InitSpriteRigidBody::Static
}
