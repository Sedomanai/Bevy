// Import necessary Bevy modules and external plugins.
use bevy::{
    camera::visibility::RenderLayers,
    prelude::*,
    winit::{UpdateMode, WinitSettings},
};

use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig};
use bevy_framepace::FramepacePlugin;

pub fn init_core_plugins(app: &mut bevy::app::App) {
    // Configure the FPS overlay display.
    let config = FpsOverlayConfig {
        text_config: TextFont {
            font_size: FontSize::Px(16.0),
            ..default()
        },
        text_color: Color::WHITE,
        enabled: true,
        refresh_interval: std::time::Duration::from_millis(100),
        frame_time_graph_config: FrameTimeGraphConfig::default(),
    };

    // Add default Bevy plugins and configure the primary window.
    // Set present mode to Mailbox for low-latency rendering without vsync.
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            present_mode: bevy::window::PresentMode::Mailbox, // No waiting for monitor
            ..default()
        }),
        ..default()
    }))
    .add_plugins((
        // Add the FramepacePlugin for advanced frame pacing control (e.g., capping FPS).
        FramepacePlugin,
        // Add the FpsOverlayPlugin with the custom configuration to display FPS.
        FpsOverlayPlugin { config },
    ))
    // Register the setup system to run once at application startup.
    .add_systems(Startup, setup);
}

pub fn init_editor_plugins(app: &mut bevy::app::App) {
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            present_mode: bevy::window::PresentMode::Immediate, // For editor, TODO: experiment
            ..default()
        }),
        ..default()
    }))
    // Configure Bevy to only update on events
    .insert_resource(WinitSettings {
        focused_mode: UpdateMode::reactive(std::time::Duration::from_secs(5)),
        unfocused_mode: UpdateMode::reactive_low_power(std::time::Duration::from_secs(10)),
    })
    // Register the setup system to run once at application startup.
    .add_systems(Startup, setup);
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            order: 32,
            ..default()
        },
        RenderLayers::layer(12),
    ));
}

pub fn orthographic_projection() -> Projection {
    Projection::Orthographic(OrthographicProjection {
        scaling_mode: bevy::camera::ScalingMode::FixedVertical {
            viewport_height: 10.0,
        },
        scale: 1.0,
        near: -1000.0,
        far: 1000.0,
        ..OrthographicProjection::default_3d()
    })
}
