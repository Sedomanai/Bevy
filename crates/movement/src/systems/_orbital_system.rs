use crate::components::{Orbiter, Tracker};
use bevy::prelude::*;

pub fn update_orbit(mut query: Query<(&mut Transform, &Orbiter, Option<&mut Tracker>)>) {
    for (mut tr, orbiter, mut tracker) in query.iter_mut() {
        let rotation = orbiter.quaternion();
        let final_pos = orbiter.focal_point + rotation * (Vec3::Z * orbiter.radius.max(0.0));

        let tr: &mut Transform = match tracker {
            Some(ref mut tracker) => &mut tracker.transform,
            None => &mut tr,
        };

        if tr.translation != final_pos {
            tr.translation = final_pos;
        }

        if orbiter.look_at && tr.rotation != rotation {
            tr.rotation = rotation;
        }
    }
}
