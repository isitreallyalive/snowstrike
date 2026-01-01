#import bevy_sprite::mesh2d_vertex_output::VertexOutput

const PI: f32 = 3.14159265359;
const DARKNESS: f32 = 0.3;

@group(2) @binding(0) var u_texture: texture_2d<f32>;
@group(2) @binding(1) var u_sampler: sampler;
@group(2) @binding(2) var<uniform> strength: f32;

fn gaussian_distribution(x: f32, stdev: f32) -> f32 {
    return exp(-(x * x) / (2.0 * stdev * stdev)) / (sqrt(2.0 * PI) * stdev);
}

fn gaussian_blur(uv: vec2<f32>, pixel_size: vec2<f32>, sigma: f32, radius: i32) -> vec3<f32> {
    var blurred_pixel = vec3<f32>(0.0);
    var total_weight = 0.0;
    
    for (var i: i32 = -radius; i <= radius; i++) {
        for (var j: i32 = -radius; j <= radius; j++) {
            let offset = vec2<f32>(f32(i), f32(j)) * pixel_size;
            let sample_pos = uv + offset;
            let weight = gaussian_distribution(f32(i), sigma) * gaussian_distribution(f32(j), sigma);
            
            blurred_pixel += textureSample(u_texture, u_sampler, sample_pos).rgb * weight;
            total_weight += weight;
        }
    }
    
    return blurred_pixel / total_weight;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;

    if strength == 0.0 {
        return textureSample(u_texture, u_sampler, uv);
    }

    let tex_size = vec2<f32>(textureDimensions(u_texture, 0));
    let pixel_size = 1.0 / tex_size;    

    let radius = i32(round(3.0 * strength));
    let blurred = gaussian_blur(uv, pixel_size, strength, radius);    
    let color = mix(blurred, vec3<f32>(0.0), DARKNESS);
    
    return vec4<f32>(color, 1.0);
}