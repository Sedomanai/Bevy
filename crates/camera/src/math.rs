#[inline]
pub fn tan_fov(fov: f32) -> f32 {
    2.0 * (fov * 0.5).tan()
}

#[inline]
pub fn ortho_from_radius(radius: f32, fov: f32) -> f32 {
    radius * tan_fov(fov)
}

#[inline]
pub fn radius_from_ortho(ortho_scale: f32, fov: f32) -> f32 {
    ortho_scale / tan_fov(fov)
}
