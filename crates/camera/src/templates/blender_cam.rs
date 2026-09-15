use bevy::{input::mouse::MouseWheel, prelude::*};
use sonolil_movement::Tracking;

use super::basic::*;
use crate::projection::*;
use crate::CameraMouseInfo;

#[cfg(feature = "hub")]
use sonolil_hub::*;

use sonolil_movement::{orbital::Orbiter, tracker::Tracker};

#[derive(Bundle)]
pub struct BlenderCameraBundle {
    tracking: TrackingCameraBundle,
    orbiter: Orbiter,
    camera3d: Camera3d,
}

#[derive(Debug, Component, Copy, Clone, PartialEq, Eq, Reflect)]
pub struct BlenderCamera;

pub fn spawn_blender_cams(mut commands: Commands) {
    let blender_camera_pos = Vec3::new(7.358, 4.958, 7.358);
    let transform = Transform::from_translation(blender_camera_pos).looking_at(Vec3::ZERO, Vec3::Y);

    let tracking_bundle = TrackingCameraBundle::new(
        transform,
        Tracker::default(),
        BlendProjection::new_persp(blender_camera_pos.length()),
    );

    commands.spawn((
        BlenderCameraBundle {
            tracking: tracking_bundle,
            orbiter: Orbiter::new(blender_camera_pos, Vec3::ZERO, true),
            camera3d: Camera3d::default(),
        },
        BlenderCamera,
        #[cfg(feature = "hub")]
        tags::EngineCamera,
    ));
}

pub fn update_blender_camera(
    time: Res<Time>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut mouse_scroll: MessageReader<MouseWheel>,
    keys: Res<ButtonInput<KeyCode>>,
    mut camera: Query<
        (
            &mut Tracker,
            &mut Orbiter,
            &mut Projection,
            &CameraMouseInfo,
            &BlenderCamera,
        ),
        (With<Camera>, With<BlenderCamera>),
    >,
    windows: Query<&Window>,
) {
    for (mut tracker, mut orbiter, mut projection, info, blender) in camera.iter_mut() {
        let blend = match &mut *projection {
            Projection::Custom(blend) => blend.get_mut::<BlendProjection>(),
            _ => None,
        };
        let Some(viewport_height) = info.viewport_height(&windows) else {
            continue;
        };

        let sum: f32 = mouse_scroll.read().map(|mw| mw.y).sum();
        let scroll_step = sum as i32;

        let swing =
            keys.any_pressed([KeyCode::AltLeft]) && mouse_button_input.pressed(MouseButton::Left);

        if mouse_button_input.pressed(MouseButton::Middle) {
            let left = tracker.transform.left();
            let up = tracker.transform.up();
            let delta = info.move_delta();
            let mut dir = left * delta.x + up * delta.y;
            dir *= if let Some(blend) = blend {
                blend.zoom_scale()
            } else {
                1.0
            };

            let mult = pan_multiplier(&projection, viewport_height);
            orbiter.focal_point += dir * mult;
            tracker.snap = 2;
        } else if scroll_step != 0 {
            orbiter.zoom(scroll_step, 0.125);

            if let Some(blend) = blend {
                blend.sync_zoom_to_focus(orbiter.radius);
            }
        } else if swing {
            //let mut delta: Vec2 = mouse_move.read().map(|mm| mm.delta).sum();

            let mut delta = info.move_delta();
            delta *= -1.0;
            let smoothed = delta * time.delta_secs() * 0.35;
            //println!(smoothed);
            orbiter.orbit(smoothed);
            //tracker.snap = 2;
        }
    }
}
