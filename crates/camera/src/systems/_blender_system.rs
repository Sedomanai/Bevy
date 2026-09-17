use crate::{_util::projection::pan_multiplier, BlendProjection, BlenderCamera, CameraMouseInfo};
use bevy::{input::mouse::MouseWheel, prelude::*};

use sonolil_movement::{Orbiter, Tracker};

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
        ),
        (With<Camera>, With<BlenderCamera>),
    >,
    windows: Query<&Window>,
) {
    for (mut tracker, mut orbiter, mut projection, info) in camera.iter_mut() {
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
            let mut delta = info.move_delta();
            delta *= -1.0;
            let smoothed = delta * time.delta_secs() * 0.35;
            orbiter.orbit(smoothed);
            //tracker.snap = 2;
        }
    }
}
