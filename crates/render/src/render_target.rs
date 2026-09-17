use crate::traits::ImageExt;
use bevy::{camera::RenderTarget, prelude::*};

#[derive(Component, Default)]
pub struct RenderTargetToImage;

pub fn render_target_to_image(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    renderables: Query<Entity, With<RenderTargetToImage>>,
) {
    for e in renderables {
        let image = Image::quick_new();
        let handle = images.add(image);

        commands
            .entity(e)
            .insert(RenderTarget::Image(handle.clone().into()));

        commands.entity(e).remove::<RenderTargetToImage>();
    }
}
