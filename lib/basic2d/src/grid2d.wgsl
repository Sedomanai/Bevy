#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct GridMaterial {
    cell_size: f32,
    subdivisions: f32,
    line_width: f32,
    grid_color: vec4<f32>,
    subdivider_color: vec4<f32>,
    axis_color: vec4<f32>,
}

@group(2) @binding(0) var<uniform> grid: GridMaterial;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let world_pos = in.world_position.xy;

    // Derivative-based pixel size for consistent line thickness across zoom levels
    let world_derivatives = fwidth(world_pos);
    let pixel_size = max(world_derivatives.x, world_derivatives.y);
    let half_line = grid.line_width * 0.5 * pixel_size;

    // --- Sub-grid calculations ---
    let sub_cell = grid.cell_size / grid.subdivisions;
    let sub_dist = abs(fract((world_pos / sub_cell) + 0.5) - 0.5) * sub_cell;
    let sub_line = min(sub_dist.x, sub_dist.y);
    let sub_alpha = 1.0 - smoothstep(half_line - pixel_size, half_line + pixel_size, sub_line);

    // --- Main grid calculations ---
    let main_dist = abs(fract((world_pos / grid.cell_size) + 0.5) - 0.5) * grid.cell_size;
    let main_line = min(main_dist.x, main_dist.y);
    let main_alpha = 1.0 - smoothstep(half_line - pixel_size, half_line + pixel_size, main_line);

    // --- Main X/Y Axis highlighting ---
    let axis_dist = abs(world_pos);
    let axis_line = min(axis_dist.x, axis_dist.y);
    let axis_alpha = 1.0 - smoothstep(half_line * 1.5 - pixel_size, half_line * 1.5 + pixel_size, axis_line);

    // Combine layers: Base -> Subdivisions -> Main Grid -> Axes
    var color = vec4<f32>(0.0);
    color = mix(color, grid.subdivider_color, sub_alpha * grid.subdivider_color.a);
    color = mix(color, grid.grid_color, main_alpha * grid.grid_color.a);

    // Highlight X and Y axes distinctly
    if (axis_dist.x < half_line * 1.5) {
        color = mix(color, vec4<f32>(0.2, 0.7, 0.2, 0.8), axis_alpha); // Y axis (Green)
    } else if (axis_dist.y < half_line * 1.5) {
        color = mix(color, vec4<f32>(0.7, 0.2, 0.2, 0.8), axis_alpha); // X axis (Red)
    }

    if (color.a <= 0.0) {
        discard;
    }

    return color;
}