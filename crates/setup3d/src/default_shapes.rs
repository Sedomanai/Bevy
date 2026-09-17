use bevy::prelude::*;

#[derive(Default)]
pub enum DefaultShapeColor {
    #[default]
    None,
    Red,
    Blue,
    Green,
}

#[derive(Resource)]
pub struct DefaultShapeFactory {
    mat: Handle<StandardMaterial>,
    mat_red: Handle<StandardMaterial>,
    mat_blue: Handle<StandardMaterial>,
    mat_green: Handle<StandardMaterial>,
    cube: Handle<Mesh>,
    sphere: Handle<Mesh>,
}

impl DefaultShapeFactory {
    pub fn sphere_bundle(&self, color: DefaultShapeColor) -> impl Bundle {
        (Mesh3d(self.sphere.clone()), self.mat_by_color(color))
    }
    pub fn cube_bundle(&self, color: DefaultShapeColor) -> impl Bundle {
        (Mesh3d(self.cube.clone()), self.mat_by_color(color))
    }

    fn mat_by_color(&self, color: DefaultShapeColor) -> MeshMaterial3d<StandardMaterial> {
        match color {
            DefaultShapeColor::None => MeshMaterial3d::<StandardMaterial>(self.mat.clone()),
            DefaultShapeColor::Red => MeshMaterial3d::<StandardMaterial>(self.mat_red.clone()),
            DefaultShapeColor::Blue => MeshMaterial3d::<StandardMaterial>(self.mat_blue.clone()),
            DefaultShapeColor::Green => MeshMaterial3d::<StandardMaterial>(self.mat_green.clone()),
        }
    }
}

pub fn register_shapes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(DefaultShapeFactory {
        mat: materials.add(Color::srgb(0.6, 0.6, 0.6)),
        mat_red: materials.add(Color::srgb(0.8, 0.4, 0.4)),
        mat_green: materials.add(Color::srgb(0.4, 0.8, 0.4)),
        mat_blue: materials.add(Color::srgb(0.4, 0.4, 0.8)),
        cube: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        sphere: meshes.add(Sphere::new(1.0).mesh()),
    });
}
