#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

const PI: f32 = 3.14159265359;

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct BlurEffect {
    strength: f32,
    darkness: f32
}

@group(0) @binding(2) var<uniform> settings: BlurEffect;

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
            
            blurred_pixel += textureSample(screen_texture, texture_sampler, sample_pos).rgb * weight;
            total_weight += weight;
        }
    }
    
    return blurred_pixel / total_weight;
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;

    if settings.strength == 0.0 {
        return textureSample(screen_texture, texture_sampler, uv);
    }

    let tex_size = vec2<f32>(textureDimensions(screen_texture, 0));
    let pixel_size = 1.0 / tex_size;    

    let radius = i32(round(3.0 * settings.strength));
    let blurred = gaussian_blur(uv, pixel_size, settings.strength, radius);    
    let color = mix(blurred, vec3<f32>(0.0), settings.darkness);
    
    return vec4<f32>(color, 1.0);
}