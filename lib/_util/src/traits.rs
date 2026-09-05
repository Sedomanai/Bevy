use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
};

pub trait ImageExt {
    fn quick_new() -> Image;
}

impl ImageExt for Image {
    fn quick_new() -> Image {
        let size = Extent3d {
            width: 1920,
            height: 1080,
            ..default()
        };

        let mut image = Image::new_fill(
            size,
            TextureDimension::D2,
            &[0, 0, 0, 255], // Black opaque pixels
            TextureFormat::Rgba8UnormSrgb,
            bevy::asset::RenderAssetUsages::default(), // or RenderAssetUsages::RENDER_WORLD
        );

        image.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
            | TextureUsages::COPY_DST
            | TextureUsages::COPY_SRC
            | TextureUsages::RENDER_ATTACHMENT;

        image
    }
}
