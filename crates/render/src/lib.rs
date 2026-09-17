use bevy::prelude::*;

mod render_target;
pub use render_target::RenderTargetToImage;

mod traits;

pub mod system_set;

#[cfg(not(feature = "hub"))]
pub use bevy::prelude::PostStartup as MatchRenderSchedule;
#[cfg(feature = "hub")]
pub use sonolil_hub::schedule::PostProcessTaggedSchedule as MatchRenderSchedule;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            MatchRenderSchedule,
            render_target::render_target_to_image.in_set(system_set::RenderTargetSet),
        );

        // app.add_systems(EguiPrimaryContextPass, dock_ui_system)
        //     .add_systems(
        //         Startup,
        //         sonolil_render::render_target_to_image::<tags::MainWorldCamera>
        //             .in_set(pass::StartupProcess),
        //     );
    }
}
