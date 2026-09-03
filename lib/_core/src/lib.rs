// Import necessary Bevy modules and external plugins.
use bevy::{
    camera::visibility::RenderLayers,
    prelude::*,
    winit::{UpdateMode, WinitSettings},
};

use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig};
use bevy_framepace::FramepacePlugin;

/// Initializes core Bevy plugins for the main application, including window settings,
/// FPS overlay, and frame pacing.
pub fn init_core_plugins(app: &mut bevy::app::App) {
    // Configure the FPS overlay display.
    // Defines custom settings for the FPS overlay, including text style, color, and refresh rate.
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

    // Adds default Bevy plugins and configures the primary window.
    // Sets present mode to Mailbox for low-latency rendering without vsync, prioritizing responsiveness.
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            present_mode: bevy::window::PresentMode::Mailbox, // No waiting for monitor
            ..default()
        }),
        ..default()
    }))
    // Integrates FramepacePlugin for advanced frame pacing control (e.g., capping FPS),
    // and FpsOverlayPlugin with custom configuration to display performance metrics.
    .add_plugins((FramepacePlugin, FpsOverlayPlugin { config }))
    // Register the setup system to run once at application startup.
    .add_systems(Startup, setup);
}

/// Initializes plugins specifically for the editor, configuring window settings
/// for immediate rendering and event-driven updates to optimize resource usage.
pub fn init_editor_plugins(app: &mut bevy::app::App) {
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            present_mode: bevy::window::PresentMode::Immediate, // For editor, TODO: experiment
            ..default()
        }),
        ..default()
    }))
    // Configures Bevy to only update on events, optimizing for editor interactivity and power efficiency.
    // Focused mode updates reactively every 5 seconds, unfocused mode every 10 seconds.
    .insert_resource(WinitSettings {
        focused_mode: UpdateMode::reactive(std::time::Duration::from_secs(5)),
        unfocused_mode: UpdateMode::reactive_low_power(std::time::Duration::from_secs(10)),
    })
    // Register the setup system to run once at application startup.
    .add_systems(Startup, setup);
}

/// Sets up a basic 2D camera with a specific render layer (12) for potential layering
/// or editor-specific views, making it distinct from other cameras.
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

/// Creates an orthographic projection suitable for 2D views or editor cameras.
/// Configured for fixed vertical scaling and specific near/far planes for depth control.
pub fn orthographic_projection() -> Projection {
    Projection::Orthographic(OrthographicProjection {
        // Sets the scaling mode to maintain a fixed vertical viewport height,
        // ensuring consistent aspect ratio regardless of window size changes.
        scaling_mode: bevy::camera::ScalingMode::FixedVertical {
            viewport_height: 10.0,
        },
        scale: 1.0,
        near: -1000.0,
        far: 1000.0,
        ..OrthographicProjection::default_3d()
    })
}
