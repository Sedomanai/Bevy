use bevy::{
    input::mouse::AccumulatedMouseScroll,
    prelude::*,
    window::{RequestRedraw, Window},
    winit::{UpdateMode, WinitSettings},
};

use moonshine_core::prelude::*;

/// A component that defines a 2D camera with pan and zoom capabilities.
///
/// This struct holds the camera's state, including its current focus point,
/// current and target zoom scales, and a reference to its transform instance.
#[derive(Component)]
pub struct PanZoom2dCamera {
    /// The world-space coordinate that the camera is currently focused on.
    pub focus: Vec2,
    /// The current scale (zoom level) of the camera.
    pub scale: f32,
    /// The target scale (zoom level) that the camera is smoothly moving towards.
    pub target_scale: f32,
    /// A placeholder for an instance of `Transform`, possibly for external interaction.
    pub to: Instance<Transform>,
    /// Stores the last known screen-space cursor position for calculating pan deltas.
    pub last_cursor_pos: Option<Vec2>,
}

impl Default for PanZoom2dCamera {
    /// Provides the default configuration for `PanZoom2dCamera`.
    ///
    /// Initializes the camera with a zero focus, a scale of 1.0, and no stored cursor position.
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
    /// Smoothly interpolates the camera's scale and focus towards their target values.
    ///
    /// This method updates the camera's current `scale` and the `translation` of the
    /// associated `Transform` component to gradually approach `target_scale` and `focus`
    /// respectively, based on the provided `smoothing` factor.
    /// It returns `true` if any interpolation occurred, indicating the need for a redraw,
    /// and `false` otherwise if the camera has reached its target state.
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

    /// Adjusts the camera's target scale based on a scroll delta input.
    ///
    /// The zoom operation is multiplicative, providing a natural feeling of increasing
    /// or decreasing zoom levels regardless of the current scale depth.
    /// The `delta` value typically comes from mouse scroll input, where positive values
    /// zoom in and negative values zoom out. The `target_scale` is clamped to prevent
    /// extreme zoom levels.
    fn zoom(&mut self, delta: f32) {
        if delta != 0.0f32 {
            // Multiplicative factor scales zoom naturally regardless of current scale depth
            let zoom_factor = 1.0 - delta * 0.1;
            self.target_scale = (self.target_scale * zoom_factor).clamp(0.1, 10.0);
        }
    }

    /// Pans the camera by updating its `focus` based on the difference between the current
    /// and last known cursor positions.
    ///
    /// This method calculates the world-space movement delta from screen-space cursor movement.
    /// It requires the current `Camera` and `OrthographicProjection` to convert screen
    /// coordinates to world coordinates accurately. The `last_cursor_pos` is updated
    /// for the next frame's calculation.
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

/// A Bevy system that provides pan and zoom functionality for entities with a `PanZoom2dCamera` component.
///
/// This system queries for `PanZoom2dCamera` components along with their `Camera`, `Projection`,
/// and `Transform` components. It processes user input from mouse scroll for zooming and
/// mouse drag (specifically the left mouse button while the `Space` key is pressed) for panning.
/// The camera's movement and scaling are smoothly interpolated based on the `WinitSettings` update mode,
/// allowing for continuous or reactive updates. It also handles resetting the `last_cursor_pos`
/// when the left mouse button is released to prevent erroneous panning when starting a new drag.
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
/// A setup system that spawns a default 2D camera with `PanZoom2dCamera` capabilities.
///
/// This function is intended to be added to a Bevy app to initialize a camera
/// entity that can be panned and zoomed. The camera is configured with a
/// `Camera2d` bundle and a `PanZoom2dCamera` component, positioned with a
/// default `Transform` to ensure 2D content is properly visible.
pub fn setup(mut commands: Commands) {
    commands.spawn((
        sonolil_core::MainCamera,
        // Default camera transform, positioned far back in Z to ensure 2D content is visible
        Transform {
            translation: Vec3::new(0.0, 0.0, 500.0),
            ..default()
        },
        Camera2d::default(),
        PanZoom2dCamera::default(),
    ));
}
