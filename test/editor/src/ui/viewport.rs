use bevy::{camera, prelude::*, render::render_resource::Extent3d};
use bevy_egui::{EguiTextureHandle, EguiUserTextures};
use sonolil_util::*;

pub struct ViewerPlugin;

impl Plugin for ViewerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            render_target_to_viewport.in_set(pass::StartupSubProcess),
        )
        .add_systems(Update, resize_texture_system);
    }
}

#[derive(Component, Default)]
pub struct ViewportState {
    pub texture: egui::TextureId,
    pub rect: Option<egui::Rect>,
    pub is_hovered: bool,
}

fn render_target_to_viewport(
    mut commands: Commands,
    mut camera: Query<(Entity, &mut render::RenderTargetImage)>,
    mut egui_user_textures: ResMut<EguiUserTextures>,
) {
    if let Ok((e, image)) = camera.single_mut() {
        let texture: egui::TextureId =
            egui_user_textures.add_image(EguiTextureHandle::Strong(image.handle.clone()));

        commands.entity(e).insert(ViewportState {
            texture,
            ..default()
        });
    }
}

fn resize_texture_system(
    mut images: ResMut<Assets<Image>>,
    mut camera: Query<(&render::RenderTargetImage, &ViewportState)>,
) {
    for (image, state) in camera.iter_mut() {
        if let Some(mut image) = images.get_mut(&image.handle) {
            let size = image.texture_descriptor.size;

            // if let Some(re) = state.rect {
            //     let new_width = re.width() as u32;
            //     let new_height = re.height() as u32;

            //     image.texture_descriptor.size = Extent3d {
            //         width: new_width,
            //         height: new_height,
            //         depth_or_array_layers: 1,
            //     };

            //     // 2. Reallocate CPU pixel buffer to match new dimensions (RGBA8 = 4 bytes per pixel)
            //     let new_pixel_count = (new_width * new_height) as usize;
            //     image.data = Some(vec![0u8; new_pixel_count * 4]);
            // };
            // 1. Update Extent3d
        }
    }
}
