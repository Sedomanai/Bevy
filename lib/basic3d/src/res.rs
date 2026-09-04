use bevy::{camera::Projection, prelude::*};

#[derive(Resource)]
pub struct MainWorld3dPerspective {
    orthographic: bool,
    projection: Projection,
}

impl Default for MainWorld3dPerspective {
    fn default() -> Self {
        Self {
            orthographic: false,
            projection: Projection::Perspective(PerspectiveProjection::default()),
        }
    }
}

impl MainWorld3dPerspective {
    pub fn set_orthographic(&mut self) {
        self.orthographic = true;
        self.projection = Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::camera::ScalingMode::FixedVertical {
                viewport_height: 10.0,
            },
            scale: 1.0,
            near: -1000.0,
            far: 1000.0,
            ..OrthographicProjection::default_3d()
        })
    }

    pub fn projection(&self) -> Projection {
        self.projection.clone()
    }
}
