use bevy::prelude::*;
use bevy_egui::egui;

#[derive(Component)]
#[require(sonolil_render::RenderTargetToImage)]
pub struct RenderTargetToEguiViewport;

#[derive(Component, Default)]
pub struct EguiViewportState {
    pub texture: egui::TextureId,
    pub rect: Option<egui::Rect>,
    pub is_hovered: bool,
}
