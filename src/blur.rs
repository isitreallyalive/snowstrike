use bevy::{
    prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef, sprite_render::Material2d,
};

#[derive(Clone, AsBindGroup, Asset, TypePath)]
pub struct BlurMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,
    #[uniform(2)]
    pub strength: f32,
}

impl BlurMaterial {
    pub const DEFAULT_STRENGTH: f32 = 1.8;
}

impl Material2d for BlurMaterial {
    fn fragment_shader() -> ShaderRef {
        "shader/blur.wgsl".into()
    }
}
