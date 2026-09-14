#[cfg(feature = "hub")]
use bevy::input::keyboard::Key;
use bevy::{
    input::{
        gestures::*,
        mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    },
    math::VectorSpace,
    prelude::*,
    window::{PrimaryWindow, Window},
};

#[cfg(feature = "hub")]
use sonolil_hub::*;

use sonolil_movement::{orbital::Orbiter, tracker::Tracker};

pub mod info;
pub mod math;
pub mod projection;
pub mod render_target;

#[cfg(feature = "hub")]
use sonolil_hub::schedule::SpawnTaggedSchedule as SpawnCameraPluginSchedule;
#[cfg(not(feature = "hub"))]
use PreStartup as SpawnCameraPluginSchedule;

#[cfg(feature = "hub")]
use sonolil_hub::schedule::ProcessTaggedSchedule as ProcessCameraPluginSchedule;
#[cfg(not(feature = "hub"))]
use Startup as ProcessCameraPluginSchedule;

#[cfg(feature = "hub")]
use crate::projection::{pan_multiplier, BlendProjection};

#[derive(Bundle)]
pub struct SonolilCameraBundle {
    transform: Transform,
    camera: Camera,
    tracker: Tracker,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraPluginTemplate {
    Blender,
}

pub struct CameraPlugin {
    pub starting_point: CameraPluginTemplate,
}

pub use info::CameraMouseInfo;

impl Default for CameraPlugin {
    fn default() -> Self {
        Self {
            starting_point: CameraPluginTemplate::Blender,
        }
    }
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        match self.starting_point {
            CameraPluginTemplate::Blender => {
                app.add_systems(SpawnCameraPluginSchedule, spawn_blender_cam);

                #[cfg(feature = "hub")]
                app.add_systems(Update, update_blender_camera)
            }
        };

        info::register(app);
    }
}

fn spawn_blender_cam(commands: Commands) {
    let blender_camera_pos = Vec3::new(7.358, 4.958, 7.358);
    let transform = Transform::from_translation(blender_camera_pos).looking_at(Vec3::ZERO, Vec3::Y);
    spawn_sonolil_camera(
        commands,
        SonolilCameraBundle {
            transform,
            camera: Camera::default(),
            tracker: Tracker {
                transform,
                ..default()
            },
        },
        #[cfg(feature = "hub")]
        (
            Orbiter::new(blender_camera_pos, Vec3::ZERO, true),
            CameraMouseInfo::default(),
            Camera3d::default(),
            tags::EngineCamera,
        ),
        #[cfg(not(feature = "hub"))]
        Camera3d::default(),
    );
}

pub fn spawn_sonolil_camera<T>(
    mut commands: Commands,
    sonolil_cam_bundle: SonolilCameraBundle,
    extra_bundle: T,
) -> Entity
where
    T: Bundle,
{
    let projection = Projection::custom(projection::BlendProjection::new(
        0.0,
        sonolil_cam_bundle
            .transform
            .translation
            .distance(Vec3::ZERO),
    ));
    commands
        .spawn((sonolil_cam_bundle, extra_bundle, projection))
        .id()
}

#[cfg(feature = "hub")]
pub fn update_blender_camera(
    time: Res<Time>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut mouse_scroll: MessageReader<MouseWheel>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (
            &mut Tracker,
            &mut Orbiter,
            &mut Projection,
            &CameraMouseInfo,
        ),
        (With<Camera>, With<tags::EngineCamera>),
    >,
    windows: Query<&Window>,
) {
    for (mut tracker, mut orbiter, mut projection, info) in query.iter_mut() {
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
            delta.y *= -1.0;
            orbiter.orbit(delta * time.delta_secs() * 0.35);
        }
    }
}
