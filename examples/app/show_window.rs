use bevy::prelude::*;
use sonolil_setup3d::default_shapes::{DefaultShapeColor, DefaultShapeFactory};

fn main() {
    let mut app = App::new();

    app.add_plugins(sonolil_app::AppPlugin {
        fps_overlay: true,
        ..default()
    })
    .add_plugins(sonolil_setup3d::Setup3dPlugin::default())
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
