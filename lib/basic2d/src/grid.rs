use bevy::{
    app::Plugin,
    asset::uuid,
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderType},
    shader::ShaderRef,
    sprite_render::{Material2d, Material2dPlugin, MeshMaterial2d},
};

use sonolil_util::*;

// Add the InfiniteGridPlugin for 3D debugging with a visual grid.
// Configures startup systems to initialize the 2D camera and grid, and registers the camera pan/zoom system for continuous updates.
// app.add_systems(Startup, (cam::setup, grid2d::setup))
//     .add_systems(Update, cam::pan_zoom_camera_system);
pub struct Grid2dPlugin;

impl Plugin for Grid2dPlugin {
    fn build(&self, app: &mut App) {
        init_shaders(app);
        app.add_plugins(Material2dPlugin::<InfiniteGrid2dMaterial>::default())
            .add_systems(Startup, init.in_set(pass::StartupSpawn));
    }
}

/// Represents the uniform data passed to the grid shader.
///
/// This struct defines the various properties that control the appearance and behavior
/// of the infinite grid, such as cell size, subdivision lines, and colors.
#[derive(ShaderType, Debug, Clone)]
struct GridMaterialUniform {
    pub cell_size: f32,
    pub subdivisions: f32,
    pub line_width: f32,
    pub grid_color: LinearRgba,
    pub subdivider_color: LinearRgba,
    pub axis_color: LinearRgba,
}

/// A Bevy `Material2d` implementation for rendering an infinite grid.
///
/// This material uses the `GridMaterialUniform` to customize the grid's appearance
/// and is designed to work with a custom shader (`grid2d.wgsl`).
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct InfiniteGrid2dMaterial {
    #[uniform(0)]
    pub grid: GridMaterialUniform,
}

/// A static `Handle<Shader>` with a unique UUID for the grid shader.
///
/// This handle is used to reference the `grid2d.wgsl` shader throughout the application,
/// ensuring it can be uniquely identified and retrieved from the asset server.
const GRID_SHADER_HANDLE: Handle<Shader> = Handle::Uuid(
    uuid::uuid!("93021948-1029-4812-8492-019284019283"),
    std::marker::PhantomData,
);

impl Material2d for InfiniteGrid2dMaterial {
    /// Specifies the fragment shader for this material.
    ///
    /// It returns a `ShaderRef` pointing to the `GRID_SHADER_HANDLE`,
    /// which references the custom `grid2d.wgsl` shader.
    fn fragment_shader() -> ShaderRef {
        GRID_SHADER_HANDLE.into()
        //ShaderRef::from(include_str!("grid2d.wgsl"))
        //"shaders/grid2d.wgsl".into()
    }

    /// Defines the alpha mode for this material.
    ///
    /// Set to `Blend` to allow the grid lines to render with transparency.
    fn alpha_mode(&self) -> bevy::sprite_render::AlphaMode2d {
        bevy::sprite_render::AlphaMode2d::Blend
    }
}

/// Initializes the necessary plugins and registers the grid shader.
///
/// This function adds the `Material2dPlugin` for `InfiniteGrid2dMaterial`
/// and inserts the `grid2d.wgsl` shader source code into the asset server
/// using the predefined `GRID_SHADER_HANDLE`.
fn init_shaders(app: &mut bevy::app::App) {
    // 2. static UUID Handle에 WGSL 셰이더 소스코드 등록
    let mut shaders = app.world_mut().resource_mut::<Assets<Shader>>();
    let res = shaders.insert(
        &GRID_SHADER_HANDLE,
        Shader::from_wgsl(include_str!("grid2d.wgsl"), "shaders/grid2d.wgsl"),
    );
    res.expect("Shader not found.");
}

/// Sets up and spawns the infinite grid entity into the Bevy world.
///
/// This function creates a large `Rectangle` mesh and applies the `InfiniteGrid2dMaterial`
/// with default grid settings, positioning it behind other sprites.
fn init(
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
