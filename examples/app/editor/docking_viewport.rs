use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;

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
    .add_plugins(sonolil_setup3d::Setup3dPlugin(
        Plugin3dSettings::GRID | Plugin3dSettings::LIGHT | Plugin3dSettings::SHAPES,
    ))
    .add_plugins(sonolil_camera::CameraPlugin(
        sonolil_camera::CameraPluginTemplate::Blender,
    ))
    .add_plugins(sonolil_editor::EditorPlugin);

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
        commands
            .entity(e)
            .insert(components::RenderTargetToEguiViewport);
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

    let EditorUiState {
        dock_state,
        counter,
    } = &mut *ui_state;

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
        });
    });

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
