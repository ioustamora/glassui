@group(0) @binding(0) var t_final: texture_2d<f32>;
@group(0) @binding(1) var s_sampler: sampler;

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

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(t_final, s_sampler, in.uv).rgb;
    // Tone mapping (Reinhard or ACES)
    // Simple reinhard:
    let mapped = color / (color + vec3<f32>(1.0));
    // Gamma correction
    let gamma = 2.2;
    return vec4<f32>(pow(mapped, vec3<f32>(1.0 / gamma)), 1.0);
}
