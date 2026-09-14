use bevy::{
    dev_tools::infinite_grid::{InfiniteGrid, InfiniteGridPlugin, InfiniteGridSettings},
    prelude::*,
};

use sonolil_util::*;

pub mod default_shapes;

#[cfg(feature = "hub")]
use sonolil_hub::schedule::SpawnTaggedSchedule as Plugin3dSchedule;

#[cfg(not(feature = "hub"))]
use Startup as Plugin3dSchedule;

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Resource)]
    pub struct Plugin3dSettings(u32) {
        const GRID    = 0b0001;
        const SHAPES  = 0b0010;
        const LIGHT   = 0b0100;
    }
}

#[derive(Deref, DerefMut)]
pub struct Setup3dPlugin(pub Plugin3dSettings);

impl Default for Setup3dPlugin {
    fn default() -> Self {
        Self(Plugin3dSettings(0b0111))
    }
}

impl Plugin for Setup3dPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "hub")]
        {
            app.add_plugins(sonolil_hub::Hub);
        }

        app.add_plugins(sonolil_camera::CameraPlugin);

        if self.contains(Plugin3dSettings::GRID) {
            app.add_plugins(InfiniteGridPlugin)
                .add_systems(Plugin3dSchedule, spawn_grid);
        }

        if self.contains(Plugin3dSettings::SHAPES) {
            app.add_systems(Plugin3dSchedule, default_shapes::register_shapes);
        }

        if self.contains(Plugin3dSettings::LIGHT) {
            app.add_systems(Plugin3dSchedule, spawn_lights);
        }
    }
}

fn spawn_grid(mut commands: Commands) {
    // // Blender's (X: 7.358, Y: -6.925, Z: 4.958) converted to Bevy's Y-Up coordinates:
    commands.spawn((InfiniteGrid, InfiniteGridSettings::default()));
}

fn spawn_lights(mut commands: Commands) {
    commands.spawn((PointLight::default(), Transform::from_xyz(2.0, 4.0, 2.0)));
    // let mut tr = Transform::from_xyz(4.0, 8.0, 4.0);
    // tr.look_at(Vec3::ZERO, Vec3::Y);
    // commands.spawn((DirectionalLight::default(), tr));
}
