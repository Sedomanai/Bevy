use bevy::{prelude::*, reflect::enums::Enum};
use egui_dock::{DockState, NodeIndex, TabViewer};
use sonolil_editor::EguiViewportState;

#[derive(Reflect, Resource, Debug, Clone, PartialEq, Eq)]
pub enum DockTab {
    Inspector,
    Console,
    Viewport,
    Custom,
}

// 2. Define the TabViewer to render content based on the active tab
pub struct MyTabViewer<'a> {
    pub counter: &'a mut u32,
    pub main_view_state: &'a mut EguiViewportState,
}

impl<'a> TabViewer for MyTabViewer<'a> {
    type Tab = DockTab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.variant_name().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            DockTab::Inspector => {
                ui.heading("Inspector Panel");
                if ui.button("Increment Counter").clicked() {
                    *self.counter += 1;
                }
                ui.label(format!("Shared Count: {}", self.counter));
            }
            DockTab::Console => {
                ui.heading("Console Logs");
                ui.label("System active. Listening for events...");
            }
            DockTab::Viewport => {
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
            DockTab::Custom => {
                ui.heading("Custom Window");
            }
        }
    }

    // just testing
    fn context_menu(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab, _path: egui_dock::NodePath) {
        match tab {
            DockTab::Viewport => {
                if ui.button("Reset View").clicked() {
                    println!("Reset view {}", _path.surface.0);
                    ui.close_menu();
                }
            }
            _ => {
                if ui.button("Close Tab").clicked() {
                    println!("Close {}", _path.surface.0);
                    // Perform tab-specific close cleanup if needed
                    ui.close_menu();
                }
            }
        }
    }
}

#[derive(Resource)]
pub struct EditorUiState {
    pub dock_state: DockState<DockTab>,
    pub counter: u32,
}

impl Default for EditorUiState {
    fn default() -> Self {
        // Create initial layout tree
        let mut dock_state = DockState::new(vec![DockTab::Viewport]);

        // Split the viewport tab: create Inspector on the right, Console on the bottom
        let surface = dock_state.main_surface_mut();
        let [left_node, _] = surface.split_right(NodeIndex::root(), 0.75, vec![DockTab::Inspector]);
        let _ = surface.split_below(left_node, 0.7, vec![DockTab::Console]);

        Self {
            dock_state,
            counter: 0,
        }
    }
}
