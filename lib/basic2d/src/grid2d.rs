use bevy::{
    asset::uuid,
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderType},
    shader::ShaderRef,
    sprite_render::{Material2d, Material2dPlugin, MeshMaterial2d},
};

#[derive(ShaderType, Debug, Clone)]
pub struct GridMaterialUniform {
    pub cell_size: f32,
    pub subdivisions: f32,
    pub line_width: f32,
    pub grid_color: LinearRgba,
    pub subdivider_color: LinearRgba,
    pub axis_color: LinearRgba,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct InfiniteGrid2dMaterial {
    #[uniform(0)]
    pub grid: GridMaterialUniform,
}

// Handle::Weak 가 아니라 Handle::Uuid 를 사용합니다.
pub const GRID_SHADER_HANDLE: Handle<Shader> = Handle::Uuid(
    uuid::uuid!("93021948-1029-4812-8492-019284019283"),
    std::marker::PhantomData,
);

impl Material2d for InfiniteGrid2dMaterial {
    fn fragment_shader() -> ShaderRef {
        GRID_SHADER_HANDLE.into()
        //ShaderRef::from(include_str!("grid2d.wgsl"))
        //"shaders/grid2d.wgsl".into()
    }

    fn alpha_mode(&self) -> bevy::sprite_render::AlphaMode2d {
        bevy::sprite_render::AlphaMode2d::Blend
    }
}

pub fn init_plugins(app: &mut bevy::app::App) {
    app.add_plugins(Material2dPlugin::<InfiniteGrid2dMaterial>::default());

    // 2. static UUID Handle에 WGSL 셰이더 소스코드 등록
    let mut shaders = app.world_mut().resource_mut::<Assets<Shader>>();
    let res = shaders.insert(
        &GRID_SHADER_HANDLE,
        Shader::from_wgsl(include_str!("grid2d.wgsl"), "shaders/grid2d.wgsl"),
    );
    res.expect("Shader not found.");
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<InfiniteGrid2dMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1000000.0, 1000000.0))),
        MeshMaterial2d(materials.add(InfiniteGrid2dMaterial {
            grid: GridMaterialUniform {
                cell_size: 100.0,
                subdivisions: 5.0, // 20px sub-cells
                line_width: 1.5,   // 1.5 pixels wide regardless of zoom
                grid_color: Color::srgba(0.4, 0.4, 0.4, 0.5).into(),
                subdivider_color: Color::srgba(0.3, 0.3, 0.3, 0.35).into(),
                axis_color: Color::srgba(0.8, 0.8, 0.8, 1.0).into(),
            },
        })),
        Transform::from_xyz(0.0, 0.0, -10.0), // Negative Z keeps it behind sprites
    ));
}
