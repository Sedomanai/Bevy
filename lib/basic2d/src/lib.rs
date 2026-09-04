use bevy::app::{Plugin, PluginGroup, PluginGroupBuilder};
use bevy::prelude::*;
use sonolil_util::*;

pub mod cam;
mod grid;

pub struct Basic2DPlugins {
    pub draw_grid: bool,
    pub pan_zoom: bool,
}

impl Default for Basic2DPlugins {
    fn default() -> Self {
        Self {
            draw_grid: false,
            pan_zoom: false,
        }
    }
}

impl PluginGroup for Basic2DPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>().add(Basic2DPlugin);

        if self.pan_zoom {
            builder = builder.add(cam::PanZoom2dCameraPlugin);
        }

        if self.draw_grid {
            builder = builder.add(grid::Grid2dPlugin);
        }

        builder
    }
}

struct Basic2DPlugin;

impl Plugin for Basic2DPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init.in_set(pass::StartupProcess));
    }
}

pub fn init(mut commands: Commands, mut cam: Query<Entity, With<tags::MainWorldCamera>>) {
    if let Ok(e) = cam.single_mut() {
        commands.entity(e).insert((
            Camera2d::default(),
            Transform {
                translation: Vec3::new(0.0, 0.0, 500.0),
                ..default()
            },
        ));
    }
}
