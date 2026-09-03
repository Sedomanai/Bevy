use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::prelude::*;
use bevy::window::{CursorOptions, Window};
use moonshine_core::prelude::*;

/// A component that controls a 3D camera with dolly, orbit, and pan functionalities.
/// It manages the camera's target arcball, sensitivity, and interaction state.
#[derive(Component)]
pub struct DollyCamera {
    /// The target arcball configuration that the camera smoothly interpolates towards.
    pub target_arc: Arcball,
    /// Sensitivity for camera movements (orbit, pan).
    pub sensitivity: f32,
    /// An optional instance to track a 'from' position for camera setup.
    pub from: Instance<Transform>,
    /// An optional instance to track a 'to' position (focus target) for camera setup.
    pub to: Instance<Transform>,
    /// The quaternion representing the pan plane's orientation.
    pub pan_plane: Quat,
    /// Stores the last cursor position for pan calculations, especially for orthographic projection.
    pub last_cursor_pos: Option<Vec2>,
}

/// Represents the spherical coordinates and focus point for an arcball camera.
/// It defines the camera's position relative to a central focus point.
#[derive(Component)]
pub struct Arcball {
    /// The 3D point in world space that the camera is looking at.
    pub focus: Vec3,
    /// The distance from the focus point to the camera.
    pub radius: f32,
    /// The yaw (horizontal rotation) of the camera around the focus point.
    pub yaw: f32,
    /// The pitch (vertical rotation) of the camera around the focus point.
    pub pitch: f32,
}

impl Default for Arcball {
    /// Creates a new `Arcball` with default values.
    /// Focus is at the origin, radius is 15.0, and yaw/pitch are 0.
    fn default() -> Self {
        Self {
            focus: Vec3::ZERO,
            radius: 15.0,
            yaw: 0.0,
            pitch: 0.0,
        }
    }
}

impl Arcball {
    /// Smoothly interpolates the current arcball state towards a `target` arcball state.
    /// The `smoothing` factor determines the speed of interpolation.
    pub fn lerp(&mut self, target: &Arcball, smoothing: f32) {
        self.yaw = self.yaw.lerp(target.yaw, smoothing);
        self.pitch = self.pitch.lerp(target.pitch, smoothing);
        self.radius = self.radius.lerp(target.radius, smoothing);
        self.focus = self.focus.lerp(target.focus, smoothing);
    }

    /// Applies the calculated position and orientation of the arcball to a given `Transform`.
    /// The camera's translation is set to the calculated position, and it looks towards the focus.
    pub fn apply_transform(&mut self, transform: &mut Transform) {
        transform.translation = self.calculate_position();
        transform.look_at(self.focus, Vec3::Y);
    }

    /// Calculates the 3D world position of the camera based on the current focus, radius, yaw, and pitch.
    /// This uses spherical coordinates to determine the camera's location.
    pub fn calculate_position(&self) -> Vec3 {
        let rotation = Quat::from_euler(EulerRot::YXZ, self.yaw, self.pitch, 0.0);
        let offset = rotation * Vec3::new(0.0, 0.0, self.radius);
        self.focus + offset
    }
}

impl Default for DollyCamera {
    /// Creates a new `DollyCamera` with default settings.
    /// Initializes `target_arc` to `Arcball::default()`, sets `sensitivity`, and placeholders for `from`/`to` instances.
    fn default() -> Self {
        Self {
            target_arc: Arcball::default(),
            sensitivity: 0.005,
            from: Instance::PLACEHOLDER,
            to: Instance::PLACEHOLDER,
            pan_plane: Quat::NAN,
            last_cursor_pos: Option::None,
        }
    }
}

impl DollyCamera {
    /// Sets the camera's `from` position, adjusting the arcball's radius, yaw, and pitch accordingly.
    /// This effectively reorients the camera to look at its current focus from the new `from` point.
    pub fn from(&mut self, from: Vec3) {
        let offset = from - self.target_arc.focus;

        self.target_arc.radius = offset.length();
        self.target_arc.yaw = offset.z.atan2(offset.x);
        self.target_arc.pitch = if self.target_arc.radius > f32::EPSILON {
            (offset.y / self.target_arc.radius).asin()
        } else {
            0.0
        };
    }

    /// Adjusts the camera's zoom level based on a `delta` value.
    /// Positive `delta` zooms in, negative zooms out. Prevents radius from going below a minimum.
    fn zoom(&mut self, delta: f32) {
        if delta != 0.0f32 {
            self.target_arc.radius -= delta * self.target_arc.radius * 0.1;
            self.target_arc.radius = self.target_arc.radius.max(0.1); // Prevent going through zero
        }
    }

    /// Orbits the camera around its focus point based on `motion_delta` (e.g., mouse movement).
    /// Updates yaw and pitch, clamping pitch to prevent camera flipping.
    fn orbit(&mut self, motion_delta: &Vec2) {
        self.target_arc.yaw -= motion_delta.x * self.sensitivity;
        self.target_arc.pitch -= motion_delta.y * self.sensitivity;

        // Clamp pitch to avoid flipping upside down over the poles
        let max_pitch = std::f32::consts::FRAC_PI_2 - 0.01;
        self.target_arc.pitch = self.target_arc.pitch.clamp(-max_pitch, max_pitch);
    }

    /// Pans the camera's focus point based on `motion_delta` and camera projection type.
    /// Handles perspective and orthographic panning differently to maintain intuitive movement.
    fn pan(
        &mut self,
        motion_delta: &Vec2,
        arcball: &mut Arcball,
        cam: &Camera,
        projection: &Projection,
        tr: &Transform,
        cursor_pos: Vec2,
    ) {
        let viewport_height = match cam.logical_viewport_size() {
            Some(size) => size.y,
            None => return,
        };

        match projection {
            Projection::Perspective(proj) => {
                // Perspective: View-aligned panning (uses pitch + yaw)
                let rotation = if self.pan_plane.is_nan() {
                    Quat::from_euler(EulerRot::YXZ, arcball.yaw, arcball.pitch, 0.0)
                } else {
                    self.pan_plane
                };

                let right = rotation * Vec3::X;
                let up = rotation * Vec3::Y;

                // Trigonometric perspective pan speed scaling based on FOV & distance
                let speed = 2.0 * (proj.fov * 0.5).tan() * arcball.radius / viewport_height;

                let delta = (-right * motion_delta.x + up * motion_delta.y) * speed;
                self.target_arc.focus += delta;
                arcball.focus += delta;
            }

            Projection::Orthographic(_proj) => {
                if let Some(last_pos) = self.last_cursor_pos {
                    let delta_cursor = cursor_pos - last_pos;

                    // Only compute if the cursor actually moved
                    if delta_cursor.length_squared() > 0.0 {
                        if let (Some(viewport_size), Projection::Orthographic(ortho)) =
                            (cam.logical_viewport_size(), &projection)
                        {
                            // world units per screen pixel
                            let world_per_pixel = ortho.area.width() / viewport_size.x;

                            let right = tr.right();
                            let up = tr.up();

                            let pan_delta =
                                (-right * delta_cursor.x + up * delta_cursor.y) * world_per_pixel;

                            self.target_arc.focus += pan_delta;
                            arcball.focus += pan_delta;
                        }
                    }
                }

                // Store the screen-space cursor position for the next frame
                self.last_cursor_pos = Some(cursor_pos);
            }
            _ => return,
        };
    }
}

/// The main Bevy system responsible for updating `DollyCamera`s.
/// It handles mouse input for zooming, orbiting, and panning, and interpolates camera transformations.
pub fn dolly_camera_system(
    time: Res<Time>,
    mut cameras: Query<(
        &mut DollyCamera,
        &Camera,
        &Projection,
        &mut Transform,
        &mut Arcball,
    )>,
    mut window: Query<(&Window, &mut CursorOptions)>,
    transforms: Query<&Transform, Without<DollyCamera>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let dt = time.delta_secs();
    let scroll_delta = mouse_scroll.delta.y;
    let motion_delta = mouse_motion.delta;
    // Check if either Left or Right Alt is held down
    let is_alt_pressed = keyboard.pressed(KeyCode::AltLeft) || keyboard.pressed(KeyCode::AltRight);

    for (mut dolly, cam, proj, mut transform, mut arcball) in cameras.iter_mut() {
        dolly.zoom(scroll_delta);

        let Ok((window, mut _cursors)) = window.single_mut() else {
            continue;
        };

        let to = transforms.get(dolly.to.into());
        let from = transforms.get(dolly.from.into());

        to.ok().map(|to| dolly.target_arc.focus = to.translation);
        from.ok().map(|from| dolly.from(from.translation));

        if is_alt_pressed && mouse_buttons.pressed(MouseButton::Left) {
            dolly.from = Instance::PLACEHOLDER;
            dolly.orbit(&motion_delta);
        }

        // --- On Drag Release ---
        if mouse_buttons.just_released(MouseButton::Middle) {
            dolly.last_cursor_pos = None;
        }

        if mouse_buttons.pressed(MouseButton::Middle) {
            dolly.from = Instance::PLACEHOLDER;
            dolly.to = Instance::PLACEHOLDER;

            let Some(cursor_pos) = window.cursor_position() else {
                continue;
            };

            dolly.pan(
                &motion_delta,
                &mut arcball,
                &cam,
                &proj,
                &transform,
                cursor_pos,
            );
        }

        arcball.lerp(&dolly.target_arc, 10.0 * dt);
        arcball.apply_transform(&mut transform);
    }
}

/// Sets up a default `DollyCamera` with an orthographic projection.
/// This function should be added to the Bevy app's `Startup` systems.
pub fn setup(mut commands: Commands) {
    let mut cam = DollyCamera::default();
    cam.from(Vec3::new(7.35, -6.92, 4.95));
    commands.spawn((
        Camera3d::default(),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::camera::ScalingMode::FixedVertical {
                viewport_height: 10.0,
            },
            scale: 1.0,
            near: -1000.0,
            far: 1000.0,
            ..OrthographicProjection::default_3d()
        }),
        cam,
        Arcball::default(),
    ));
}
