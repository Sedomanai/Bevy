use bevy::app::{PluginGroup, PluginGroupBuilder};
use bevy::prelude::*;
use sonolil_util::*;

mod cam;
mod grid;
mod res;

pub struct Basic3DPlugins {
    pub orthographic: bool,
    pub draw_grid: bool,
    pub dolly: bool,
}

impl Default for Basic3DPlugins {
    fn default() -> Self {
        Self {
            orthographic: false,
            draw_grid: false,
            dolly: false,
        }
    }
}

impl PluginGroup for Basic3DPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>().add(Basic3DPlugin {
            orthographic: self.orthographic,
        });

        if self.dolly {
            builder = builder.add(cam::DollyCameraPlugin)
        }

        if self.draw_grid {
            builder = builder.add(grid::GridPlugin)
        }

        builder
    }
}

struct Basic3DPlugin {
    orthographic: bool,
}

impl Plugin for Basic3DPlugin {
    fn build(&self, app: &mut App) {
        let mut perspective = res::MainWorld3dPerspective::default();
        if self.orthographic {
            perspective.set_orthographic();
        }

        app.insert_resource(perspective)
            .add_systems(Startup, init.in_set(pass::StartupProcess));
    }
}

pub fn init(
    mut commands: Commands,
    mut cam: Query<Entity, With<tags::MainWorldCamera>>,
    target_perspective: ResMut<res::MainWorld3dPerspective>,
) {
    if let Ok(e) = cam.single_mut() {
        commands.entity(e).insert((
            Camera3d::default(),
            Transform {
                translation: Vec3::new(0.0, 0.0, 500.0),
                ..default()
            },
            target_perspective.projection(),
        ));
    }
}
