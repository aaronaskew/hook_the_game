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
            .insert_resource(LevelSelection::Index(0))
            .add_systems(OnEnter(GameState::LoadingLevel), setup);
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

    state.set(GameState::SpawningPlayer);
}

// #[derive(Bundle, LdtkEntity)]
// pub struct MyBundle {
//     a: ComponentA,
//     b: ComponentB,
//     #[sprite_sheet_bundle]
//     sprite_bundle: SpriteSheetBundle,
// }
