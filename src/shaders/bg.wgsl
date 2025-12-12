struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(i32(in_vertex_index) - 1);
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.uv = vec2<f32>(x, y) * 0.5 + 0.5;
    out.uv.y = 1.0 - out.uv.y; 
    return out;
}

struct Uniforms {
    time: f32,
    resolution: vec2<f32>,
};
@group(0) @binding(0) var<uniform> uniforms: Uniforms;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    
    // Dark deep background
    var color = vec3<f32>(0.02, 0.02, 0.05);
    
    // Moving grid
    let grid_scale = 20.0;
    // Fix aspect ratio
    let aspect = uniforms.resolution.x / uniforms.resolution.y;
    let grid_uv = uv * vec2<f32>(grid_scale * aspect, grid_scale) + vec2<f32>(uniforms.time * 0.1, uniforms.time * 0.05);
    
    let grid_x = abs(fract(grid_uv.x) - 0.5);
    let grid_y = abs(fract(grid_uv.y) - 0.5);
    let line_width = 0.02;
    
    let grid_val = smoothstep(0.5 - line_width, 0.5, grid_x) + smoothstep(0.5 - line_width, 0.5, grid_y);
    
    // Neon glow grid
    let glow = vec3<f32>(0.0, 0.8, 1.0) * grid_val * 0.2;
    
    // Vignette
    let dist = distance(uv, vec2<f32>(0.5)) * 1.5;
    color += glow * (1.0 - dist);
    
    return vec4<f32>(color, 1.0);
}
