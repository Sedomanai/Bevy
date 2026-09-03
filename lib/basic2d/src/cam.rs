use bevy::{
    input::mouse::AccumulatedMouseScroll,
    prelude::*,
    window::{RequestRedraw, Window},
    winit::{UpdateMode, WinitSettings},
};

use moonshine_core::prelude::*;

#[derive(Component)]
pub struct PanZoom2dCamera {
    pub focus: Vec2,
    pub scale: f32,
    pub target_scale: f32,
    pub to: Instance<Transform>,
    pub last_cursor_pos: Option<Vec2>,
}

impl Default for PanZoom2dCamera {
    fn default() -> Self {
        Self {
            focus: Vec2::ZERO,
            scale: 1.0,
            target_scale: 1.0,
            to: Instance::PLACEHOLDER,
            last_cursor_pos: None,
        }
    }
}

impl PanZoom2dCamera {
    fn lerp(&mut self, tr: &mut Transform, smoothing: f32) -> bool {
        let mut draw = false;

        if (self.scale - self.target_scale).abs() < 0.001 {
            self.target_scale = self.scale;
        } else {
            self.scale = self.scale.lerp(self.target_scale, smoothing);
            draw = true;
        }

        let f = Vec3::new(self.focus.x, self.focus.y, 0.0);
        if tr.translation.distance(f) < 0.001 {
            tr.translation = f;
        } else {
            tr.translation.x = tr.translation.x.lerp(f.x, smoothing);
            tr.translation.y = tr.translation.y.lerp(f.y, smoothing);
            draw = true;
        }

        return draw;
    }

    fn zoom(&mut self, delta: f32) {
        if delta != 0.0f32 {
            // Multiplicative factor scales zoom naturally regardless of current scale depth
            let zoom_factor = 1.0 - delta * 0.1;
            self.target_scale = (self.target_scale * zoom_factor).clamp(0.1, 10.0);
        }
    }

    fn pan(&mut self, cam: &Camera, projection: &mut OrthographicProjection, cursor_pos: Vec2) {
        if let Some(last_pos) = self.last_cursor_pos {
            let mut delta_cursor = cursor_pos - last_pos;

            // Only compute if the cursor actually moved
            if delta_cursor.length_squared() > 0.0 {
                if let Some(viewport_size) = cam.logical_viewport_size() {
                    // world units per screen pixel
                    let world_per_pixel = projection.area.width() / viewport_size.x;

                    delta_cursor.x *= -1.0;
                    let pan_delta = delta_cursor * world_per_pixel;

                    self.focus += pan_delta;
                }
            }
        }

        // Store the screen-space cursor position for the next frame
        self.last_cursor_pos = Some(cursor_pos);
    }
}

pub fn pan_zoom_camera_system(
    time: Res<Time>,
    mut cameras: Query<(
        &mut PanZoom2dCamera,
        &Camera,
        &mut Projection,
        &mut Transform,
    )>,
    mut window: Query<&Window>,
    winit_settings: Res<WinitSettings>,
    _transforms: Query<&Transform, Without<PanZoom2dCamera>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    keyboard: Res<ButtonInput<KeyCode>>, // Added to check for Alt key)
    mut _redraw_events: MessageWriter<RequestRedraw>,
) {
    let dt = time.delta_secs();
    let scroll_delta = mouse_scroll.delta.y;
    let space = keyboard.pressed(KeyCode::Space);
    //let is_alt_pressed = keyboard.pressed(KeyCode::AltLeft) || keyboard.pressed(KeyCode::AltRight);

    for (mut panzoom, cam, mut proj, mut transform) in cameras.iter_mut() {
        let Projection::Orthographic(ref mut ortho) = *proj else {
            continue;
        };

        let Ok(window) = window.single_mut() else {
            continue;
        };

        panzoom.zoom(scroll_delta);

        if mouse_buttons.just_released(MouseButton::Left) {
            panzoom.last_cursor_pos = None;
        }

        if space && mouse_buttons.pressed(MouseButton::Left) {
            let Some(cursor_pos) = window.cursor_position() else {
                return;
            };
            panzoom.to = Instance::PLACEHOLDER;
            panzoom.pan(&cam, ortho, cursor_pos);
        }

        let current_mode = if window.focused {
            &winit_settings.focused_mode
        } else {
            &winit_settings.unfocused_mode
        };

        match current_mode {
            UpdateMode::Reactive { .. } => {
                let tr = &mut transform.translation;
                let focus = &mut panzoom.focus;
                *tr = Vec3::new(focus.x, focus.y, tr.z);
                ortho.scale = panzoom.target_scale;
            }
            UpdateMode::Continuous => {
                if panzoom.lerp(&mut transform, 10.0 * dt) {
                    ortho.scale = panzoom.scale;
                }
            }
        }
    }
}

// Setup snippet to add to your app:
pub fn setup(mut commands: Commands) {
    commands.spawn((
        Transform {
            translation: Vec3::new(0.0, 0.0, 500.0),
            ..default()
        },
        Camera2d::default(),
        PanZoom2dCamera::default(),
    ));
}
