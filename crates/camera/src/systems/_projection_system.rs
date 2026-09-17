use crate::components::BlendProjection;
use bevy::prelude::*;

pub fn pan_multiplier(proj: &Projection, viewport_height: f32) -> f32 {
    match proj {
        Projection::Perspective(_) => 10.0, // ?
        Projection::Orthographic(ortho) => ortho.area.height() / viewport_height,
        Projection::Custom(custom) => {
            if let Some(blend_proj) = custom.get::<BlendProjection>() {
                blend_proj.pan_ratio(viewport_height)
            } else {
                10.0
            }
        }
    }
}
