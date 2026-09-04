use bevy::{
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
};
use bevy_egui::{EguiTextureHandle, EguiUserTextures};

pub struct ViewerPlugin;

impl Plugin for ViewerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_viewport);
    }
}

#[derive(Resource, Deref)]
pub struct ViewportImage(Handle<Image>);

fn setup_viewport(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut egui_user_textures: ResMut<EguiUserTextures>,
    mut ui_state: ResMut<super::docks::EditorUiState>, // <- add this
) {
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    image.resize(size);

    // add to image to assets, and its handle to egui_textures and global resource
    let image_handle = images.add(image);
    let tex_id = egui_user_textures.add_image(EguiTextureHandle::Strong(image_handle.clone()));
    ui_state.viewport_texture = Some(tex_id);

    commands.insert_resource(ViewportImage(image_handle.clone()));
}
