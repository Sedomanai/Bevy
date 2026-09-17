use bevy::prelude::*;
use egui_dock::{DockState, NodeIndex, TabViewer};
use sonolil_editor::EguiViewportState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tab {
    Inspector,
    Console,
    Viewport,
}

// 2. Define the TabViewer to render content based on the active tab
pub struct MyTabViewer<'a> {
    pub counter: &'a mut u32,
    pub main_view_state: &'a mut EguiViewportState,
}

impl<'a> TabViewer for MyTabViewer<'a> {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            Tab::Inspector => "Inspector".into(),
            Tab::Console => "Console".into(),
            Tab::Viewport => "Viewport".into(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            Tab::Inspector => {
                ui.heading("Inspector Panel");
                if ui.button("Increment Counter").clicked() {
                    *self.counter += 1;
                }
                ui.label(format!("Shared Count: {}", self.counter));
            }
            Tab::Console => {
                ui.heading("Console Logs");
                ui.label("System active. Listening for events...");
            }
            Tab::Viewport => {
                ui.heading("3D Viewport");
                let panel_size = ui.available_size();
                let texture_size = egui::vec2(1920.0, 1080.0);

                let uv_width = (panel_size.x / texture_size.x).min(1.0);
                let uv_height = (panel_size.y / texture_size.y).min(1.0);

                let half_uv_w = uv_width / 2.0;
                let half_uv_h = uv_height / 2.0;

                let uv_min = egui::pos2(0.5 - half_uv_w, 0.5 - half_uv_h);
                let uv_max = egui::pos2(0.5 + half_uv_w, 0.5 + half_uv_h);

                let render_size = egui::vec2(
                    panel_size.x.min(texture_size.x),
                    panel_size.y.min(texture_size.y),
                );

                ui.add(
                    egui::Image::new((self.main_view_state.texture, render_size))
                        .uv(egui::Rect::from_min_max(uv_min, uv_max)),
                );
            }
        }
    }
}

#[derive(Resource)]
pub struct EditorUiState {
    pub dock_state: DockState<Tab>,
    pub counter: u32,
}

impl Default for EditorUiState {
    fn default() -> Self {
        // Create initial layout tree
        let mut dock_state = DockState::new(vec![Tab::Viewport]);

        // Split the viewport tab: create Inspector on the right, Console on the bottom
        let surface = dock_state.main_surface_mut();
        let [left_node, _] = surface.split_right(NodeIndex::root(), 0.75, vec![Tab::Inspector]);
        let _ = surface.split_below(left_node, 0.7, vec![Tab::Console]);

        Self {
            dock_state,
            counter: 0,
        }
    }
}
