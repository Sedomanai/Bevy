use bevy::prelude::*;
use bevy_egui::EguiPlugin;

pub mod components;
pub use components::*;

use sonolil_render::RenderPlugin;

mod systems;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin::default());
        }
        if !app.is_plugin_added::<RenderPlugin>() {
            app.add_plugins(RenderPlugin);
        }

        systems::register_systems(app);

        // app.add_systems(EguiPrimaryContextPass, dock_ui_system)
        //     .add_systems(
        //         Startup,
        //         sonolil_render::render_target_to_image::<tags::MainWorldCamera>
        //             .in_set(pass::StartupProcess),
        //     );
    }
}
