pub fn init(
    mut commands: Commands,
    mut main_cam: Query<Entity, (With<tags::MainCamera>, With<Camera>)>,
    mut sub_cam: Query<Entity, (With<tags::SubCamera>, With<Camera>)>,
    mut debug_cam: Query<Entity, (With<tags::DebugCamera>, With<Camera>)>,
) {
    // Debug Cam
    if let Ok(e) = debug_cam.single_mut() {
        let mut e = commands.entity(e);
        e.insert(Camera {
            order: 32,
            ..default()
        });
        e.insert(RenderLayers::layer(32));
    }
}
