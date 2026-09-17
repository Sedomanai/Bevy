use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
};

pub trait ImageExt {
    fn quick_new() -> Image;
    fn resize_and_reallocate_pixels(&mut self, width: u32, height: u32);
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

    fn resize_and_reallocate_pixels(&mut self, width: u32, height: u32) {
        self.texture_descriptor.size = Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        // 2. Reallocate CPU pixel buffer to match new dimensions (RGBA8 = 4 bytes per pixel)
        let pixel_count = (width * height) as usize;
        self.data = Some(vec![0u8; pixel_count * 4]);
    }
}
