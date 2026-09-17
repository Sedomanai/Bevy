use bevy::prelude::*;
use sonolil_setup3d::{
    Plugin3dSettings,
    default_shapes::{DefaultShapeColor, DefaultShapeFactory},
};

fn main() {
    let mut app = App::new();

    app.add_plugins(sonolil_app::AppPlugin {
        fps_overlay: true,
        ..default()
    })
    .add_plugins(sonolil_setup3d::Setup3dPlugin(
        Plugin3dSettings::GRID | Plugin3dSettings::LIGHT | Plugin3dSettings::SHAPES,
    ))
    .add_plugins(sonolil_camera::CameraPlugin(
        sonolil_camera::CameraPluginTemplate::Blender,
    ))
    .add_systems(Startup, setup)
    .run();
}

fn setup(mut commands: Commands, shapes: Option<Res<DefaultShapeFactory>>) {
    let Some(shapes) = shapes else {
        return;
    };
    commands.spawn((
        shapes.cube_bundle(DefaultShapeColor::None),
        Transform::default(),
    ));
}
