use bevy::{
    prelude::*,
    window::WindowMode,
    winit::{UpdateMode, WinitSettings},
};

use bevy::app::{Plugin, PluginGroup};
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig};
use bevy_framepace::FramepacePlugin;

pub struct AppPlugin {
    pub full_screen: bool,
    pub reactive_update_mode: bool,
    pub fps_overlay: bool,
}

impl Default for AppPlugin {
    fn default() -> Self {
        Self {
            full_screen: false,
            reactive_update_mode: false,
            fps_overlay: false,
        }
    }
}

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        let present_mode = if self.reactive_update_mode {
            bevy::window::PresentMode::Immediate // Reactive
        } else {
            bevy::window::PresentMode::Mailbox
        };

        let mode: WindowMode = if self.full_screen {
            WindowMode::BorderlessFullscreen(MonitorSelection::Current)
        } else {
            WindowMode::Windowed
        };

        // Adds default Bevy plugins and configures the primary window.
        // Sets present mode to Mailbox for low-latency rendering without vsync, prioritizing responsiveness.
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode,
                mode,
                ..default()
            }),
            ..default()
        }));

        if self.fps_overlay {
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

            app
                // Integrates FramepacePlugin for advanced frame pacing control (e.g., capping FPS),
                // and FpsOverlayPlugin with custom configuration to display performance metrics.
                .add_plugins(FpsOverlayPlugin { config });
        }

        if self.reactive_update_mode {
            app.insert_resource(WinitSettings {
                focused_mode: UpdateMode::reactive(std::time::Duration::from_secs(5)),
                unfocused_mode: UpdateMode::reactive_low_power(std::time::Duration::from_secs(10)),
            });
            // Register the setup
        } else {
            app.add_plugins(FramepacePlugin);
        }
    }
}
