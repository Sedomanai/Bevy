#[inline]
pub fn half_tan_fov(fov: f32) -> f32 {
    (fov * 0.5).tan()
}

#[inline]
pub fn tan_fov(fov: f32) -> f32 {
    2.0 * half_tan_fov(fov)
}

#[inline]
pub fn ortho_from_focal_distance(focal_distance: f32, fov: f32) -> f32 {
    focal_distance * tan_fov(fov).max(0.001)
}

#[inline]
pub fn focal_distance_from_ortho(ortho_scale: f32, fov: f32) -> f32 {
    ortho_scale / tan_fov(fov).max(0.001)
}
