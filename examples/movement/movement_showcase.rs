// RIGHT CLICK START TEST MOVING

use bevy::prelude::*;
use sonolil_camera::*;
use sonolil_movement::*;
use sonolil_setup3d::default_shapes::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(sonolil_app::AppPlugin {
        fps_overlay: true,
        ..default()
    })
    .add_plugins(sonolil_setup3d::Setup3dPlugin::default())
    .add_plugins(sonolil_movement::MovementPlugin)
    .add_plugins(sonolil_camera::CameraPlugin(CameraPluginTemplate::Blender))
    .add_systems(Startup, showcase)
    .add_systems(Update, rotate_tagged)
    .run();
}

#[derive(Component)]
struct TrackerTag;

#[derive(Component)]
struct CentralObject;

fn showcase(mut commands: Commands, shapes: Option<Res<DefaultShapeFactory>>) {
    let Some(shapes) = shapes else {
        return;
    };

    let orbiter = commands
        .spawn((
            shapes.cube_bundle(DefaultShapeColor::Green),
            orbital::Orbiter {
                look_at: false,
                radius: 3.5,
                ..default()
            },
            CentralObject,
            Transform {
                scale: Vec3::new(0.8, 0.8, 0.8),
                ..default()
            },
        ))
        .id();

    commands.spawn((
        shapes.cube_bundle(DefaultShapeColor::Blue),
        orbital::Orbiter {
            look_at: true,
            radius: 2.5,
            ..default()
        },
        orbital::Orbiting(orbiter),
        Transform {
            scale: Vec3::new(0.6, 0.6, 0.6),
            ..default()
        },
    ));

    commands.spawn((
        shapes.sphere_bundle(DefaultShapeColor::Red),
        tracker::Tracker {
            decay: 5.0,
            snap: 2,
            ..default()
        },
        tracker::Tracking(orbiter),
        TrackerTag,
        Transform {
            scale: Vec3::new(0.3, 0.3, 0.3),
            ..default()
        },
    ));
}

fn rotate_tagged(
    time: Res<Time>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut flag: Local<bool>,
    mut query: Query<(Has<CentralObject>, &mut orbital::Orbiter), Without<Camera>>,
) {
    // RIGHT CLICK START TEST MOVING

    if mouse.just_pressed(MouseButton::Right) {
        *flag = !*flag;
    }

    for (central, mut tagged_orbiter) in query.iter_mut() {
        let speed: f32 = if central { 2.0 } else { 5.0 };
        if central && !*flag {
            continue;
        }

        tagged_orbiter.orbit(Vec2::new(speed * time.delta_secs(), 0.0));
    }
}
