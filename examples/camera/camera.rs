use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};
use sonolil_camera::*;
use sonolil_hub::*;
use sonolil_setup3d::{Plugin3dSettings, default_shapes::DefaultShapeFactory};

fn main() {
    let mut app = App::new();

    app.add_plugins(sonolil_app::AppPlugin {
        fps_overlay: true,
        ..default()
    })
    .add_plugins(sonolil_setup3d::Setup3dPlugin(
        Plugin3dSettings::SHAPES | Plugin3dSettings::GRID | Plugin3dSettings::LIGHT,
    ))
    .add_plugins(CameraPlugin(CameraPluginTemplate::Blender))
    .add_plugins(bevy_egui::EguiPlugin::default())
    .add_systems(Startup, setup)
    .add_systems(EguiPrimaryContextPass, update_camera)
    .run();
}

fn setup(mut commands: Commands, shapes: Option<Res<DefaultShapeFactory>>) {
    let Some(shapes) = shapes else {
        return;
    };
    commands.spawn((
        shapes.cube_bundle(sonolil_setup3d::default_shapes::DefaultShapeColor::Blue),
        Transform {
            translation: Vec3::new(0.0, 0.5, 0.0),
            ..default()
        },
    ));
}

fn update_camera(
    mut contexts: EguiContexts,
    mut camera_query: Query<&mut Projection, (With<Camera>, With<tags::EngineCamera>)>,
) {
    let Ok(mut projection) = camera_query.single_mut() else {
        return;
    };

    let Some(ctx) = contexts.ctx_mut().ok() else {
        return;
    };

    egui::Window::new("Camera Controls").show(ctx, |ui| {
        // Downcast to your custom projection inside Projection::Custom
        if let Projection::Custom(custom) = projection.as_mut() {
            if let Some(blend) = custom.get_mut::<components::BlendProjection>() {
                ui.label("Projection Settings");

                // Slider binding directly to your struct's fields
                ui.add(
                    egui::Slider::new(&mut blend.blend, 0.0..=1.0)
                        .text("Blend Projection Slider")
                        .custom_formatter(|val, _| format!("{:.0}%", val * 100.0)),
                );
            }
        }
    });
}
