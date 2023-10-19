#![allow(unused)]
use crate::GameState;
use bevy::prelude::*;

// TODO wasm veilid
// #[cfg(target_arch = "wasm32")]
// use veilid_core::
// #[cfg(not(target_arch = "wasm32"))]
// use veilid_core::prelude::*;

pub struct VideoPlugin;

/// This plugin is responsible for the playing video.
/// The video is only played during the State `GameState::CutScene` and is removed when that state is exited
impl Plugin for VideoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::CutScene), setup_video)
            //.add_systems(Update, click_play_button.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::CutScene), cleanup_video);
    }
}

// #[derive(Resource)]
// struct ButtonColors {
//     normal: Color,
//     hovered: Color,
// }

// impl Default for ButtonColors {
//     fn default() -> Self {
//         ButtonColors {
//             normal: Color::rgb(0.15, 0.15, 0.15),
//             hovered: Color::rgb(0.25, 0.25, 0.25),
//         }
//     }
// }

fn setup_video(mut commands: Commands) {
    todo!("setup_video");

    // commands.spawn(Camera2dBundle::default());
    // commands
    //     .spawn(ButtonBundle {
    //         style: Style {
    //             width: Val::Px(120.0),
    //             height: Val::Px(50.0),
    //             margin: UiRect::all(Val::Auto),
    //             justify_content: JustifyContent::Center,
    //             align_items: AlignItems::Center,
    //             ..default()
    //         },
    //         background_color: button_colors.normal.into(),
    //         ..Default::default()
    //     })
    //     .with_children(|parent| {
    //         parent.spawn(TextBundle::from_section(
    //             "Play",
    //             TextStyle {
    //                 font_size: 40.0,
    //                 color: Color::rgb(0.9, 0.9, 0.9),
    //                 ..default()
    //             },
    //         ));
    //     });
}

// fn click_play_button(
//     button_colors: Res<ButtonColors>,
//     mut state: ResMut<NextState<GameState>>,
//     mut interaction_query: Query<
//         (&Interaction, &mut BackgroundColor),
//         (Changed<Interaction>, With<Button>),
//     >,
// ) {
//     for (interaction, mut color) in &mut interaction_query {
//         match *interaction {
//             Interaction::Pressed => {
//                 state.set(GameState::Playing);
//             }
//             Interaction::Hovered => {
//                 *color = button_colors.hovered.into();
//             }
//             Interaction::None => {
//                 *color = button_colors.normal.into();
//             }
//         }
//     }
// }

fn cleanup_video(mut commands: Commands, button: Query<Entity, With<Button>>) {
    // commands.entity(button.single()).despawn_recursive();
    todo!("cleanup_video");
}
