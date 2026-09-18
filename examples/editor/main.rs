use bevy::prelude::*;
use bevy::{camera::visibility::RenderLayers, reflect::enums::Enum};

use egui_dock::DockArea;

use sonolil_editor::*;
use sonolil_setup3d::*;

use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};

mod sample_ui;
use sample_ui::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(sonolil_app::AppPlugin {
        reactive_update_mode: true,
        ..default()
    })
    .add_plugins(sonolil_editor::EditorPlugin);

    app.add_plugins(sonolil_setup3d::Setup3dPlugin(
        Plugin3dSettings::GRID | Plugin3dSettings::LIGHT | Plugin3dSettings::SHAPES,
    ))
    .add_plugins(sonolil_camera::CameraPlugin(
        sonolil_camera::CameraPluginTemplate::Blender,
    ));

    app.insert_resource(sample_ui::EditorUiState::default());

    app.add_systems(Startup, cam_work)
        .add_systems(EguiPrimaryContextPass, update_egui_context)
        .run();
}

pub fn cam_work(
    mut commands: Commands,
    main_cam: Query<Entity, (With<Camera>, With<sonolil_hub::tags::EngineCamera>)>,
) {
    if let Ok(e) = main_cam.single() {
        commands.entity(e).insert(RenderTargetToEguiViewport);
    }

    // UI Cam is a must.
    // TODO: move this to camera module or something.
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

pub fn update_egui_context(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<sample_ui::EditorUiState>,
    mut main_view: Query<&mut EguiViewportState, With<sonolil_hub::tags::EngineCamera>>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    show_top_panel(ctx, &mut ui_state.dock_state);

    if let Ok(viewport_state) = main_view.single_mut() {
        show_docked_windows(ctx, viewport_state, ui_state);
    }
}

fn show_top_panel(ctx: &mut egui::Context, dock_state: &mut egui_dock::DockState<DockTab>) {
    #[allow(deprecated)]
    egui::Panel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open").clicked() {
                    // Handle open
                }
                if ui.button("Quit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

            ui.menu_button("Edit", |ui| {
                if ui.button("Undo").clicked() {
                    // Handle undo
                }
            });

            ui.menu_button("Windows", |ui| {
                let available_tabs = [
                    DockTab::Inspector,
                    DockTab::Console,
                    DockTab::Viewport,
                    DockTab::Custom,
                ];

                for tab in available_tabs {
                    if ui.button(tab.variant_name()).clicked() {
                        if let Some(tab_path) = dock_state.find_tab(&tab) {
                            // Tab already exists: bring it into focus
                            dock_state.set_active_tab(tab_path).unwrap_or_else(|e| {
                                bevy::log::warn!("Failed to set active tab: {e}")
                            });
                        } else {
                            // Tab doesn't exist: append to the focused leaf
                            dock_state.push_to_focused_leaf(tab);
                        }
                    }
                }
            });
        });
    });
}

fn show_docked_windows(
    ctx: &mut egui::Context,
    mut viewport_state: Mut<'_, EguiViewportState>,
    mut ui_state: ResMut<sample_ui::EditorUiState>,
) {
    let EditorUiState {
        counter,
        dock_state,
    } = &mut *ui_state;

    let mut tab_viewer = MyTabViewer {
        counter: counter,
        main_view_state: &mut *viewport_state,
    };
    #[allow(deprecated)]
    DockArea::new(dock_state).show(ctx, &mut tab_viewer);
}

// For reference

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
