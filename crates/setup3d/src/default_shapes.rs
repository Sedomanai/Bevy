use bevy::prelude::*;

#[derive(Resource)]
pub struct DefaultShapeFactory {
    mat: Handle<StandardMaterial>,
    cube: Handle<Mesh>,
    sphere: Handle<Mesh>,
}

impl DefaultShapeFactory {
    pub fn sphere_bundle(&self) -> impl Bundle {
        (
            Mesh3d(self.sphere.clone()),
            MeshMaterial3d(self.mat.clone()),
        )
    }
    pub fn cube_bundle(&self) -> impl Bundle {
        (Mesh3d(self.cube.clone()), MeshMaterial3d(self.mat.clone()))
    }
}

pub fn register_shapes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(DefaultShapeFactory {
        mat: materials.add(Color::srgb(0.6, 0.6, 0.6)),
        cube: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        sphere: meshes.add(Sphere::new(1.0).mesh()),
    });
}
