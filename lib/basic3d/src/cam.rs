use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::prelude::*;
use bevy::window::{CursorOptions, Window};
use moonshine_core::prelude::*;

#[derive(Component)]
pub struct DollyCamera {
    pub target_arc: Arcball,
    pub sensitivity: f32,
    pub from: Instance<Transform>,
    pub to: Instance<Transform>,
    pub pan_plane: Quat,
    pub last_cursor_pos: Option<Vec2>,
}

#[derive(Component)]
pub struct Arcball {
    pub focus: Vec3,
    pub radius: f32,
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for Arcball {
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
    pub fn lerp(&mut self, target: &Arcball, smoothing: f32) {
        self.yaw = self.yaw.lerp(target.yaw, smoothing);
        self.pitch = self.pitch.lerp(target.pitch, smoothing);
        self.radius = self.radius.lerp(target.radius, smoothing);
        self.focus = self.focus.lerp(target.focus, smoothing);
    }

    pub fn apply_transform(&mut self, transform: &mut Transform) {
        // Apply to Transform every frame
        transform.translation = self.calculate_position();
        transform.look_at(self.focus, Vec3::Y);
    }

    // Calculate new 3D position using the smoothed spherical coordinates
    pub fn calculate_position(&self) -> Vec3 {
        let rotation = Quat::from_euler(EulerRot::YXZ, self.yaw, self.pitch, 0.0);
        let offset = rotation * Vec3::new(0.0, 0.0, self.radius);
        self.focus + offset
    }
}

impl Default for DollyCamera {
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

    fn zoom(&mut self, delta: f32) {
        if delta != 0.0f32 {
            self.target_arc.radius -= delta * self.target_arc.radius * 0.1;
            self.target_arc.radius = self.target_arc.radius.max(0.1); // Prevent going through zero
        }
    }

    fn orbit(&mut self, motion_delta: &Vec2) {
        self.target_arc.yaw -= motion_delta.x * self.sensitivity;
        self.target_arc.pitch -= motion_delta.y * self.sensitivity;

        // Clamp pitch to avoid flipping upside down over the poles
        let max_pitch = std::f32::consts::FRAC_PI_2 - 0.01;
        self.target_arc.pitch = self.target_arc.pitch.clamp(-max_pitch, max_pitch);
    }

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
            None => return, // Skip if viewport isn't calculated yet
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

        //let pan_speed = arcball.radius * self.sensitivity * 0.225;
    }
}

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
    keyboard: Res<ButtonInput<KeyCode>>, // Added to check for Alt key
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

// Setup snippet to add to your app:
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
