use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use sonolil_hub::*;
pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(schedule::SpawnTaggedSchedule, init);
    }
}

fn init(
    mut commands: Commands,
    mut main_cam: Query<Entity, (With<tags::MainCamera>, With<Camera>)>,
    mut sub_cam: Query<Entity, (With<tags::SubCamera>, With<Camera>)>,
    mut debug_cam: Query<Entity, (With<tags::DebugCamera>, With<Camera>)>,
) {
    // Debug Cam
    if let Ok(e) = debug_cam.single_mut() {
        let e = commands.entity(e);
        e.insert(Camera {
            order: 32,
            ..default()
        });
        e.insert(BundleRenderLayers::layer(32));
    }
}
