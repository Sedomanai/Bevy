use bevy::prelude::*;
use sonolil_movement::*;
use sonolil_setup3d::default_shapes::DefaultShapeFactory;

fn main() {
    let mut app = App::new();

    app.add_plugins(sonolil_app::AppPlugin {
        fps_overlay: true,
        ..default()
    })
    .add_plugins(sonolil_setup3d::Setup3dPlugin::default())
    .add_plugins(sonolil_movement::MovementPlugin)
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
            shapes.cube_bundle(),
            orbital::Orbiter::new(3.5, false),
            CentralObject,
            Transform {
                scale: Vec3::new(0.5, 0.5, 0.5),
                ..default()
            },
        ))
        .id();

    commands.spawn((
        shapes.cube_bundle(),
        orbital::Orbiter::new(1.5, true),
        orbital::Orbiting(orbiter),
        Transform {
            scale: Vec3::new(0.3, 0.3, 0.3),
            ..default()
        },
    ));

    commands.spawn((
        shapes.sphere_bundle(),
        tracker::Tracker {
            decay: 5.0,
            snap: 2,
            ..default()
        },
        tracker::Tracking(orbiter),
        TrackerTag,
        Transform {
            scale: Vec3::new(0.2, 0.2, 0.2),
            ..default()
        },
    ));
}

fn rotate_tagged(
    time: Res<Time>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut flag: Local<bool>,
    mut query: Query<(Has<CentralObject>, &mut orbital::Orbiter)>,
) {
    if mouse.just_pressed(MouseButton::Left) {
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
