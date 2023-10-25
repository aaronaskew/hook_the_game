use bevy::{prelude::*, render::primitives::Aabb};
use bevy_xpbd_2d::prelude::*;
use crate::GameState;

#[derive(Component)]
pub struct Ground;

pub struct LevelPlugin;

/// This plugin handles level creation
/// Runs during `GameState::BuildingLevel`
// impl Plugin for PhysicsPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_plugins(PhysicsPlugins::default())
//             .add_systems(OnEnter(GameState::BuildingLevel), build_level);
//     }
// }

pub fn build_level(mut commands: Commands) {}
