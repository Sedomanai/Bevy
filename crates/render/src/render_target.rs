use crate::traits::ImageExt;
use bevy::{asset::Handle, camera::RenderTarget, prelude::*};

#[derive(Component, Default)]
pub struct RenderTargetImage {
    pub handle: Handle<Image>,
}

pub fn render_target_to_image<T: Component>(
    mut commands: Commands,
    mut camera: Query<Entity, With<T>>,
    mut images: ResMut<Assets<Image>>,
) where
    T: Component,
{
    if let Ok(e) = camera.single_mut() {
        let image = Image::quick_new();
        let handle = images.add(image);

        commands
            .entity(e)
            .insert(RenderTarget::Image(handle.clone().into()));
        commands.entity(e).insert(RenderTargetImage {
            handle,
            ..default()
        });
    }
}
