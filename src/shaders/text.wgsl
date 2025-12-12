struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
};

struct Uniforms {
    time: f32,
    _pad1: u32,
    resolution: vec2<f32>,
};
@group(0) @binding(0) var<uniform> uniforms: Uniforms;

@group(1) @binding(0) var t_diffuse: texture_2d<f32>;
@group(1) @binding(1) var s_diffuse: sampler;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Similar projection to glass shader, assuming input position is in pixels (Top-Left 0,0)
    let res = max(uniforms.resolution, vec2<f32>(1.0));
    
    let ndc_x = (input.position.x / res.x) * 2.0 - 1.0;
    let ndc_y = 1.0 - (input.position.y / res.y) * 2.0;
    
    out.clip_position = vec4<f32>(ndc_x, ndc_y, 0.0, 1.0);
    out.uv = input.uv;
    out.color = input.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Sample alpha from texture (R8Unorm usually appears in .r)
    // If it's single channel Red, we use .r as alpha
    let alpha = textureSample(t_diffuse, s_diffuse, in.uv).r;
    
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}
