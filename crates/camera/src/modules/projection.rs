use bevy::{camera::CameraProjection, camera::SubCameraView, prelude::*};

/// Camera projection storing ONLY the blend factor.
#[derive(Debug, Clone, Reflect)]
pub struct BlendProjection {
    /// 0.0 = Perspective (3D), 1.0 = Orthographic (2D)
    blend: f32,
    aspect_ratio: f32,
    fov: f32,
    ortho_scale: f32,
    near: f32,
    far: f32,
}

impl Default for BlendProjection {
    fn default() -> Self {
        Self {
            blend: 0.0,
            aspect_ratio: 1.0,
            fov: std::f32::consts::FRAC_PI_4, // 45 deg
            ortho_scale: 1.0,
            near: 0.05,
            far: 800.0,
        }
    }
}

impl BlendProjection {
    pub fn sync_to_radius(&mut self, radius: f32) {
        self.ortho_scale = crate::math::ortho_from_radius(radius.max(self.near), self.fov);
    }

    pub fn is_orthographic(&self) -> bool {
        self.blend >= 0.999
    }
    pub fn is_perspective(&self) -> bool {
        self.blend <= 0.001
    }
    pub fn set_blend(&mut self, value: f32) {
        self.blend = value;
        self.clamp_blend();
    }

    pub fn clamp_blend(&mut self) {
        if self.is_perspective() {
            self.blend = 0.0;
        } else if self.is_orthographic() {
            self.blend = 1.0;
        }
    }
}

// #[derive(Component, Default, Copy, Clone)]
// struct CurrBlendProjection(Vec3);

impl CameraProjection for BlendProjection {
    fn get_clip_from_view(&self) -> Mat4 {
        if self.is_perspective() {
            PerspectiveProjection {
                fov: self.fov,
                aspect_ratio: self.aspect_ratio,
                near: self.near,
                far: self.far,
                ..Default::default()
            }
            .get_clip_from_view()
        } else if self.is_orthographic() {
            OrthographicProjection {
                scale: self.ortho_scale,
                near: self.near,
                far: self.far,
                ..OrthographicProjection::default_3d()
            }
            .get_clip_from_view()
        } else {
            // pray

            let b = self.blend;

            let (sin_fov, cos_fov) = (self.fov * 0.5).sin_cos();
            let y_scale_p = cos_fov / sin_fov; // 1 / tan(fov/2)
            let x_scale_p = y_scale_p / self.aspect_ratio;
            let r_p = self.far / (self.near - self.far);

            let ortho_top = self.ortho_scale * 0.5;
            let ortho_right = ortho_top * self.aspect_ratio;
            let y_scale_o = 1.0 / ortho_top;
            let x_scale_o = 1.0 / ortho_right;
            let r_o = 1.0 / (self.near - self.far);

            let x_scale = x_scale_p.lerp(x_scale_o, b);
            let y_scale = y_scale_p.lerp(y_scale_o, b);
            let z_z = r_p.lerp(r_o, b);
            let z_w = (-1.0_f32).lerp(0.0, b);
            let w_z = (r_p * self.near).lerp(r_o * self.near, b);
            let w_w = (0.0_f32).lerp(1.0, b);

            Mat4::from_cols(
                Vec4::new(x_scale, 0.0, 0.0, 0.0),
                Vec4::new(0.0, y_scale, 0.0, 0.0),
                Vec4::new(0.0, 0.0, z_z, z_w),
                Vec4::new(0.0, 0.0, w_z, w_w),
            )
        }
    }

    fn get_clip_from_view_for_sub(&self, sub_view: &SubCameraView) -> Mat4 {
        let mut projection = self.get_clip_from_view();

        let full_x = sub_view.full_size.x as f32;
        let full_y = sub_view.full_size.y as f32;
        let size_x = sub_view.size.x as f32;
        let size_y = sub_view.size.y as f32;
        let offset_x = sub_view.offset.x as f32;
        let offset_y = sub_view.offset.y as f32;

        let scale_x = full_x / size_x;
        let scale_y = full_y / size_y;
        let shift_x = (full_x - 2.0 * offset_x - size_x) / size_x;
        let shift_y = (full_y - 2.0 * offset_y - size_y) / size_y;

        projection.x_axis.x *= scale_x;
        projection.y_axis.y *= scale_y;
        projection.z_axis.x = shift_x;
        projection.z_axis.y = shift_y;

        projection
    }

    fn update(&mut self, width: f32, height: f32) {
        if height > 0.0 {
            self.aspect_ratio = width / height;
        }
    }

    fn far(&self) -> f32 {
        self.far
    }

    fn get_frustum_corners(&self, z_near: f32, z_far: f32) -> [Vec3A; 8] {
        if self.is_perspective() {
            // Pure Perspective
            PerspectiveProjection {
                fov: self.fov,
                aspect_ratio: self.aspect_ratio,
                near: self.near,
                far: self.far,
                ..Default::default()
            }
            .get_frustum_corners(z_near, z_far)
        } else if self.is_orthographic() {
            // Pure Orthographic
            OrthographicProjection {
                scale: self.ortho_scale,
                near: self.near,
                far: self.far,
                ..OrthographicProjection::default_3d()
            }
            .get_frustum_corners(z_near, z_far)
        } else {
            let b = self.blend;

            let tan_half_fov = (self.fov * 0.5).tan();
            let ortho_top = self.ortho_scale * 0.5;
            let ortho_right = ortho_top * self.aspect_ratio;

            let half_extents_at = |z: f32| -> (f32, f32) {
                let persp_half_h = z.abs() * tan_half_fov;
                let persp_half_w = persp_half_h * self.aspect_ratio;

                let half_h = persp_half_h.lerp(ortho_top, b);
                let half_w = persp_half_w.lerp(ortho_right, b);
                (half_w, half_h)
            };

            let (w_n, h_n) = half_extents_at(z_near);
            let (w_f, h_f) = half_extents_at(z_far);

            [
                Vec3A::new(w_n, -h_n, z_near),  // near bottom right
                Vec3A::new(w_n, h_n, z_near),   // near top right
                Vec3A::new(-w_n, h_n, z_near),  // near top left
                Vec3A::new(-w_n, -h_n, z_near), // near bottom left
                Vec3A::new(w_f, -h_f, z_far),   // far bottom right
                Vec3A::new(w_f, h_f, z_far),    // far top right
                Vec3A::new(-w_f, h_f, z_far),   // far top left
                Vec3A::new(-w_f, -h_f, z_far),  // far bottom left
            ]
        }
    }
}

pub fn sync_projection(proj: &mut Projection, radius: f32) {
    if let Projection::Custom(proj) = proj {
        if let Some(proj) = proj.as_any_mut().downcast_mut::<BlendProjection>() {
            proj.sync_to_radius(radius);
        }
    }
}

pub fn pan_multiplier(proj: &Projection, viewport_height: f32) -> f32 {
    match proj {
        Projection::Perspective(_) => 10.0, // ?
        Projection::Orthographic(ortho) => ortho.area.height() / viewport_height,
        Projection::Custom(custom) => {
            if let Some(blend_proj) = custom.as_any().downcast_ref::<BlendProjection>() {
                blend_proj.ortho_scale / viewport_height
            } else {
                10.0
            }
        }
    }
}
