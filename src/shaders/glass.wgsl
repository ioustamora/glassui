struct VertexInput {
    @location(0) position: vec2<f32>, 
    @location(1) size: vec2<f32>,     
    @location(2) color: vec4<f32>,
    @location(3) corner_radius: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>, // 0..1 relative to rect
    @location(2) size: vec2<f32>, // Pixel size of the rect
    @location(3) corner_radius: f32,
};

struct Uniforms {
    time: f32,
    resolution: vec2<f32>,
};
@group(0) @binding(0) var<uniform> uniforms: Uniforms;

@group(1) @binding(0) var bg_texture: texture_2d<f32>;
@group(1) @binding(1) var bg_sampler: sampler;

@vertex
fn vs_main(
    @builtin(vertex_index) v_idx: u32,
    input: VertexInput
) -> VertexOutput {
    var pos = vec2<f32>(0.0, 0.0);
    if (v_idx == 1u || v_idx == 3u) { pos.y = 1.0; }
    if (v_idx == 2u || v_idx == 3u) { pos.x = 1.0; }
    
    var out: VertexOutput;
    let world_pos = input.position + pos * input.size;
    
    let res = max(uniforms.resolution, vec2<f32>(1.0));
    
    let ndc_x = (world_pos.x / res.x) * 2.0 - 1.0;
    let ndc_y = 1.0 - (world_pos.y / res.y) * 2.0;

    out.clip_position = vec4<f32>(ndc_x, ndc_y, 0.0, 1.0);
    out.color = input.color;
    out.uv = pos; 
    out.size = input.size;
    out.corner_radius = input.corner_radius;
    return out;
}

fn sd_rounded_box(p: vec2<f32>, b: vec2<f32>, r: f32) -> f32 {
    let q = abs(p) - b + r;
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - r;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let center_uv = in.uv - 0.5;
    let p = center_uv * in.size;
    let half_size = in.size * 0.5;
    
    // Use per-instance corner radius (0 = sharp, otherwise rounded)
    let r = min(in.corner_radius, min(half_size.x, half_size.y));
    let dist = sd_rounded_box(p, half_size, r);
    
    let alpha_mask = 1.0 - smoothstep(-0.5, 0.5, dist);
    
    if (alpha_mask <= 0.0) {
        discard;
    }

    let border_width = 2.0;
    let border_alpha = 1.0 - smoothstep(border_width - 1.0, border_width, abs(dist));
    let glow_intensity = exp(-0.1 * abs(dist)) * in.color.a * 3.0; // Increased glow
    
    // Glass Effect: Sample blurred background
    let screen_uv = in.clip_position.xy / uniforms.resolution;
    // Add distortion based on normal (fake normal from center)
    // Simple distortion: move UV towards center based on distance from center of rect
    // let distortion = center_uv * 0.05 * alpha_mask;
    // let glass_color = textureSample(bg_texture, bg_sampler, screen_uv + distortion).rgb;
    
    // For now, simple sampling
    let glass_color = textureSample(bg_texture, bg_sampler, screen_uv).rgb;
    
    // Tint
    let tint = in.color.rgb;
    // Mix glass with tint
    let mixed_color = mix(glass_color, tint, 0.3); // 30% tint, 70% background
    
    let border_col = vec3<f32>(0.7, 0.9, 1.0); 
    
    let final_rgb = mix(mixed_color, border_col, border_alpha * 0.6);
    // Add glow
    let out_col = final_rgb + border_col * glow_intensity * 0.4;
    
    // We want output alpha to be high to cover the sharp background drawn behind?
    // Actually we drew sharp BG behind. 
    // If we output alpha < 1.0, we blend with sharp BG.
    // We want to replace sharp BG with blurred version.
    // So alpha should be 1.0 where mask is 1.0?
    // But we want transparency for the *tint* part?
    // If we output alpha 1.0, we obscure the sharp BG completely with the blurred sample. That is correct.
    // But at the edges (anti-aliasing), alpha_mask < 1.0. We want to blend with sharp BG there.
    
    return vec4<f32>(out_col, alpha_mask);
}
