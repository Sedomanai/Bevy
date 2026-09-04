use bevy::{
    camera::{visibility::RenderLayers, RenderTarget},
    prelude::*,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};
use egui_dock::{DockArea, DockState, NodeIndex, TabViewer};

pub struct DockPlugin;

impl Plugin for DockPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .init_resource::<EditorUiState>()
            .add_systems(Startup, spawn_ui_cam)
            .add_systems(EguiPrimaryContextPass, dock_ui_system);
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
    viewport_texture: Option<egui::TextureId>,
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
                if let Some(tex) = self.viewport_texture {
                    ui.add(egui::Image::new((tex, ui.available_size())));
                } else {
                    ui.label("Game view goes here, no game viewport texture yet.");
                }
            }
        }
    }
}

// 3. Resource to hold the dock layout state and shared app state
#[derive(Resource)]
pub struct EditorUiState {
    pub viewport_texture: Option<egui::TextureId>,
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
            viewport_texture: Option::None,
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
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut ui_state: ResMut<EditorUiState>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    let EditorUiState {
        viewport_texture,
        dock_state,
        counter,
    } = &mut *ui_state;

    let mut tab_viewer = MyTabViewer {
        counter,
        viewport_texture: *viewport_texture,
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
