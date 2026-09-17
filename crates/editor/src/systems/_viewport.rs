use crate::components::{EguiViewportState, RenderTargetToEguiViewport};
use bevy::camera::RenderTarget;
use bevy::prelude::*;

use bevy_egui::{egui, EguiTextureHandle, EguiUserTextures};

pub fn render_target_to_viewport(
    mut commands: Commands,
    render_targets: Query<(Entity, &RenderTarget), With<RenderTargetToEguiViewport>>,
    mut user_textures: ResMut<EguiUserTextures>,
) {
    for (e, render_target) in render_targets {
        if let RenderTarget::Image(image_target) = render_target {
            let handle = &image_target.handle;

            let texture: egui::TextureId =
                user_textures.add_image(EguiTextureHandle::Strong(handle.clone()));

            commands.entity(e).insert(EguiViewportState {
                texture,
                ..default()
            });
        }

        commands.entity(e).remove::<RenderTargetToEguiViewport>();
    }
}

// Deprecated for now

// pub fn resize_texture_system(
//     mut images: ResMut<Assets<Image>>,
//     mut camera: Query<(&RenderTargetImage, &ViewportState)>,
// ) {
//     for (image, state) in camera.iter_mut() {
//         if let Some(mut image) = images.get_mut(&image.handle) {
//             let size = image.texture_descriptor.size;

//             // if let Some(re) = state.rect {
//             //     let new_width = re.width() as u32;
//             //     let new_height = re.height() as u32;

//             //     image.texture_descriptor.size = Extent3d {
//             //         width: new_width,
//             //         height: new_height,
//             //         depth_or_array_layers: 1,
//             //     };

//             //     // 2. Reallocate CPU pixel buffer to match new dimensions (RGBA8 = 4 bytes per pixel)
//             //     let new_pixel_count = (new_width * new_height) as usize;
//             //     image.data = Some(vec![0u8; new_pixel_count * 4]);
//             // };
//         }
//     }
// }
