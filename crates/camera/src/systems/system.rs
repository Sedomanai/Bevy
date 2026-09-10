// // use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
// use crate::modules::focus::*;
// use crate::modules::orbit::*;
// use crate::modules::projection;
// use crate::modules::projection::*;
// use bevy::prelude::*;
// // use bevy::window::{CursorOptions, Window};

// pub fn first_pass(
//     time: Res<Time>,
//     mut query: Query<(&Camera, &mut Projection)>,
//     mut queries: ParamSet<(
//         Query<(&Camera, &Projection)>,
//         Query<(&Camera, &mut Projection)>,
//     )>,
// ) {
//     for (cam, proj) in queries.p1().iter() {
//         projection::sync_projection(&mut proj, radius);
//     }
// }

// pub fn update(
//     time: Res<Time>,
//     mut queries: ParamSet<(
//         Query<(&Camera, &Projection)>,
//         Query<(&mut Camera, &mut Projection)>,
//         Query<(&Camera, &mut Projection)>,
//     )>,
// ) {
//     for (cam, proj) in queries.p0().iter() {}
// }

// /// Makes a given `Transform` (i.e. camera) look at the current focus point.
// pub fn look_at(&self, tr: &mut Transform) {
//     // Use Vec3::Y as the up direction for the look_at transformation
//     tr.look_at(self.focus, Vec3::Y);
// }

// /// Calculate position and orientation of the arcball to a given 'focus' and 'radius'
// /// The camera's translation is set to the calculated position. (tr.translation =)
// /// Later add offset to position of 'focus' and set it to transform.translation.
// /// They are decoupled because setting translation directly can clash with other components.
// pub fn final_position(&self, focus: &CameraFocus) -> Vec3 {
//     //, transform: &mut Transform) {
//     let rotation = Quat::from_euler(EulerRot::YXZ, self.yaw, self.pitch, 0.0);
//     let offset = rotation * Vec3::new(0.0, 0.0, self.radius); // offset
//     focus.focus + offset
// }
