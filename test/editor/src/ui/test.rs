use bevy::{camera::visibility::RenderLayers, prelude::*};
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};
use egui_dock::{DockArea, DockState, NodeIndex, TabViewer};

use crate::ui::viewport::*;
use sonolil_util::*;

pub struct DockPlugin;

impl Plugin for DockPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .init_resource::<EditorUiState>()
            .add_systems(Startup, spawn_ui_cam.in_set(pass::StartupSpawn))
            .add_systems(EguiPrimaryContextPass, dock_ui_system)
            .add_systems(
                Startup,
                render::render_target_to_image::<tags::MainWorldCamera>
                    .in_set(pass::StartupProcess),
            );
    }
}

// 1. Define the enum representing your tabs
#[derive(Debug, Clone, PartialEq, Eq)]
enum Tab {
    Inspector,
    Console,
    Viewport,
}

// 2. Define the TabViewer to render content based on the active tab
struct MyTabViewer<'a> {
    counter: &'a mut u32,
    main_view_state: &'a mut ViewportState,
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

// 3. Resource to hold the dock layout state and shared app state
#[derive(Resource)]
pub struct EditorUiState {
    dock_state: DockState<Tab>,
    counter: u32,
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

pub fn spawn_ui_cam(mut commands: Commands) {
    // This is where bevy / egui UI elements will draw
    commands.spawn((
        Camera2d,
        Camera {
            order: 31,
            ..default()
        },
        IsDefaultUiCamera,
        RenderLayers::layer(31),
    ));
}

pub fn dock_ui_system(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<EditorUiState>,
    mut main_view: Query<&mut ViewportState, With<tags::MainWorldCamera>>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    let EditorUiState {
        dock_state,
        counter,
    } = &mut *ui_state;

    if let Ok(state) = main_view.single_mut() {
        let mut tab_viewer = MyTabViewer {
            counter,
            main_view_state: state.into_inner(),
        };

        // #[allow(deprecated)]
        // egui::CentralPanel::default()
        //     .frame(egui::Frame::central_panel(&ctx.global_style()).inner_margin(0.))
        //     .show(ctx, |ui| {
        //         DockArea::new(dock_state)
        //             .style(Style::from_egui(ui.style().as_ref()))
        //             .show_inside(ui, &mut tab_viewer);
        //     });

        // #[allow(deprecated)]
        // DockArea::new(dock_state)
        //     .style(Style::from_egui(&ctx.global_style()))
        //     .show(ctx, &mut tab_viewer);

        #[allow(deprecated)]
        DockArea::new(dock_state).show(ctx, &mut tab_viewer);
    }
}
