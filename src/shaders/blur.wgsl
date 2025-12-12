@group(0) @binding(0) var input_texture: texture_2d<f32>;
@group(0) @binding(1) var output_texture: texture_storage_2d<rgba8unorm, write>;

struct Params {
    direction: vec2<u32>, // (1,0) or (0,1)
};
@group(0) @binding(2) var<uniform> params: Params;

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let dims = textureDimensions(input_texture);
    if (global_id.x >= dims.x || global_id.y >= dims.y) {
        return;
    }
    
    // Simple Box Blur or Gaussian
    // Let's do a 9-tap gaussian for simplicity and speed
    var weights = array<f32, 5>(0.227027, 0.1945946, 0.1216216, 0.054054, 0.016216);
    
    var color = textureLoad(input_texture, global_id.xy, 0).rgb * weights[0];
    
    for (var i = 1; i < 5; i++) {
        let offset = vec2<i32>(i32(params.direction.x) * i, i32(params.direction.y) * i);
        
        let sample1 = textureLoad(input_texture, vec2<u32>(vec2<i32>(global_id.xy) + offset), 0).rgb;
        let sample2 = textureLoad(input_texture, vec2<u32>(vec2<i32>(global_id.xy) - offset), 0).rgb;
        
        color += (sample1 + sample2) * weights[i];
    }
    
    textureStore(output_texture, global_id.xy, vec4<f32>(color, 1.0));
}
