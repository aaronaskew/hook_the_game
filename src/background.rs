use crate::{shader_utils::common::ShadplayShaderLibrary, GameState};
use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
    sprite::{Material2d, Material2dPlugin},
};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<YourShader2D>::default())
            .add_plugins(ShadplayShaderLibrary)
            .add_systems(OnEnter(GameState::Playing), setup_background);
    }
}

pub fn setup_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut your_shader: ResMut<Assets<YourShader2D>>,
    windows: Query<&Window>,
) {
    let win = windows
        .get_single()
        .expect("Should be impossible to NOT get a window");

    // Quad
    commands.spawn((
        bevy::sprite::MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(win.width(), win.height())).into())
                .into(),
            material: your_shader.add(YourShader2D {
                mouse_pos: MousePos {
                    x: 100.0f32,
                    y: 128.0f32,
                },
            }),
            transform: Transform::from_translation(Vec3::new(0., 0., -10.)),
            // .with_rotation(Quat::from_rotation_x(180.0)), //FIX ME to avoid the rotate2D call in all shaders..
            ..default()
        },
        // BillBoardQuad,
    ));
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "f528511f-dcf2-4b0b-9522-a9df3a1a795b"]
pub struct YourShader2D {
    #[uniform(0)]
    pub(crate) mouse_pos: MousePos,
    // #[texture(1, dimension = "2d")]
    // #[sampler(2)]
    // pub img: Handle<Image>,
}

// pub struct YourShader2D {
//     #[uniform(0)]
//     pub(crate) mouse_pos: MousePos,

//     #[texture(1, dimension = "2d")]
//     #[sampler(2)]
//     pub img: Handle<Image>,
// }

#[derive(ShaderType, Debug, Clone)]
pub struct MousePos {
    pub x: f32,
    pub y: f32,
}

impl Material2d for YourShader2D {
    fn fragment_shader() -> ShaderRef {
        "shaders/background.wgsl".into()
    }
}
