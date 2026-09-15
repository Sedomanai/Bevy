use crate::math::*;
use bevy::{camera::CameraProjection, camera::SubCameraView, prelude::*};

/// Camera projection storing ONLY the blend factor.
#[derive(Debug, Clone, Reflect)]
pub struct BlendProjection {
    /// 0.0 = Perspective (3D), 1.0 = Orthographic (2D)
    pub blend: f32,
    pub near: f32,
    pub far: f32,
    /// Zoom scale that unites the ortho and persp
    /// Used internally to sync camera distance from focal_point as zoom point
    /// Works only with CameraPlugin
    zoom_scale: f32,
    /// ortho_top or top at focal/near plane
    focal_top: f32,
    /// ortho_right or right at focal/near plane
    focal_right: f32,
    /// The distance from the camera at which ortho and persp plane projects the same frustrum
    focal_distance: f32,
    aspect_ratio: f32,
    fov: f32,
}

impl Default for BlendProjection {
    fn default() -> Self {
        Self {
            blend: 0.0,
            zoom_scale: 1.0,
            focal_distance: 10.0,
            aspect_ratio: 1.0,
            focal_top: 0.0,
            focal_right: 0.0,
            fov: std::f32::consts::FRAC_PI_4, // 45 deg
            near: 0.05,
            far: 800.0,
        }
    }
}

impl BlendProjection {
    pub fn new_ortho(focal_distance: f32) -> Self {
        BlendProjection::new(1.0, focal_distance)
    }

    pub fn new_persp(focal_distance: f32) -> Self {
        BlendProjection::new(0.0, focal_distance)
    }

    // blend 0.0 for perspective, 1.0 for ortho projection
    pub fn new(blend: f32, focal_distance: f32) -> Self {
        let mut selfie = Self { blend, ..default() };
        selfie.set_focal_distance(focal_distance);
        selfie
    }

    /// Set the distance in which ortho and persp planes overlap to the same viewport.
    ///
    /// Ortho area is recalculated from the distance.
    pub fn set_focal_distance(&mut self, distance: f32) {
        self.focal_distance = distance.max(self.near).max(0.001);

        let ortho_scale = ortho_from_focal_distance(self.focal_distance, self.fov);
        self.focal_top = ortho_scale * 0.5;
        self.focal_right = self.focal_top * self.aspect_ratio;
    }

    pub fn zoom_scale(&self) -> f32 {
        self.zoom_scale
    }

    /// Syncs zoom with current distance's ratio to focal distance, with the focal distance intact.
    pub fn sync_zoom_to_focus(&mut self, curr_dist: f32) {
        self.zoom_scale = (curr_dist / self.focal_distance).max(0.001);
    }

    pub fn is_blend_orthographic(&self) -> bool {
        self.blend >= 0.999
    }
    pub fn is_blend_perspective(&self) -> bool {
        self.blend <= 0.001
    }
    pub fn set_blend(&mut self, value: f32) {
        self.blend = value;
        self.clamp_blend();
    }

    pub fn clamp_blend(&mut self) {
        if self.is_blend_perspective() {
            self.blend = 0.0;
        } else if self.is_blend_orthographic() {
            self.blend = 1.0;
        }
    }
}

// #[derive(Component, Default, Copy, Clone)]
// struct CurrBlendProjection(Vec3);
impl CameraProjection for BlendProjection {
    fn get_clip_from_view(&self) -> Mat4 {
        if self.is_blend_perspective() {
            // There's just no way to provide it manually right now.
            PerspectiveProjection {
                fov: self.fov,
                aspect_ratio: self.aspect_ratio,
                near: self.near,
                far: self.far,
                ..Default::default()
            }
            .get_clip_from_view()
        } else if self.is_blend_orthographic() {
            let right = self.focal_right * self.zoom_scale;
            let top = self.focal_top * self.zoom_scale;
            Mat4::orthographic_rh(-right, right, -top, top, self.far, self.near)
        } else {
            // pray
            let b = self.blend;

            let (sin_fov, cos_fov) = (self.fov * 0.5).sin_cos();
            let y_scale_p = cos_fov / sin_fov;
            let x_scale_p = y_scale_p / self.aspect_ratio;

            let y_scale_o = 1.0 / (self.focal_top * self.zoom_scale);
            let x_scale_o = 1.0 / (self.focal_right * self.zoom_scale);

            let inv_range = 1.0 / (self.far - self.near);
            let z_coeff_p = self.near * inv_range; // reverse-Z perspective
            let z_coeff_o = inv_range; // reverse-Z ortho

            let x_scale = x_scale_p.lerp(x_scale_o, b);
            let y_scale = y_scale_p.lerp(y_scale_o, b);

            let z_coeff = z_coeff_p.lerp(z_coeff_o, b);
            let z_const = self.far * z_coeff; // 양 끝에서 D = far * C 로 항상 성립

            let z_w = (-1.0_f32).lerp(0.0, b);

            Mat4::from_cols(
                Vec4::new(x_scale, 0.0, 0.0, 0.0),
                Vec4::new(0.0, y_scale, 0.0, 0.0),
                Vec4::new(0.0, 0.0, z_coeff, z_w),
                Vec4::new(0.0, 0.0, z_const, b),
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
            self.focal_right = self.focal_top * self.aspect_ratio;
        }
    }

    fn far(&self) -> f32 {
        self.far
    }

    fn get_frustum_corners(&self, z_near: f32, z_far: f32) -> [Vec3A; 8] {
        if self.is_blend_perspective() {
            // Pure Perspective
            let half_tan_fov = crate::math::half_tan_fov(self.fov);
            let a = z_near.abs() * half_tan_fov;
            let b = z_far.abs() * half_tan_fov;
            let aspect_ratio = self.aspect_ratio;
            [
                Vec3A::new(a * aspect_ratio, -a, z_near),  // bottom right
                Vec3A::new(a * aspect_ratio, a, z_near),   // top right
                Vec3A::new(-a * aspect_ratio, a, z_near),  // top left
                Vec3A::new(-a * aspect_ratio, -a, z_near), // bottom left
                Vec3A::new(b * aspect_ratio, -b, z_far),   // bottom right
                Vec3A::new(b * aspect_ratio, b, z_far),    // top right
                Vec3A::new(-b * aspect_ratio, b, z_far),   // top left
                Vec3A::new(-b * aspect_ratio, -b, z_far),  // bottom left
            ]
        } else if self.is_blend_orthographic() {
            // NOTE: These are virtually copy pasted from the orthographic.
            let right = self.focal_right * self.zoom_scale;
            let top = self.focal_top * self.zoom_scale;
            [
                Vec3A::new(right, -top, z_near),  // bottom self.right
                Vec3A::new(right, top, z_near),   // self.top self.right
                Vec3A::new(-right, top, z_near),  // self.top left
                Vec3A::new(-right, -top, z_near), // bottom left
                Vec3A::new(right, -top, z_far),   // bottom self.right
                Vec3A::new(right, top, z_far),    // self.top self.right
                Vec3A::new(-right, top, z_far),   // self.top left
                Vec3A::new(-right, -top, z_far),  // bottom left
            ]
        } else {
            let b = self.blend;
            let half_extents_at = |z: f32| -> (f32, f32) {
                let depth_ratio = z.abs() / self.focal_distance;
                let persp_half_h = self.focal_top * depth_ratio;
                let persp_half_w = self.focal_right * depth_ratio;

                let ortho_half_h = self.focal_top * self.zoom_scale;
                let ortho_half_w = self.focal_right * self.zoom_scale;

                // 3. Blend between perspective and orthographic
                let half_h = persp_half_h.lerp(ortho_half_h, b);
                let half_w = persp_half_w.lerp(ortho_half_w, b);

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

pub fn pan_multiplier(proj: &Projection, viewport_height: f32) -> f32 {
    match proj {
        Projection::Perspective(_) => 10.0, // ?
        Projection::Orthographic(ortho) => ortho.area.height() / viewport_height,
        Projection::Custom(custom) => {
            if let Some(blend_proj) = custom.get::<BlendProjection>() {
                (blend_proj.focal_top * 2.0) / viewport_height
            } else {
                10.0
            }
        }
    }
}
