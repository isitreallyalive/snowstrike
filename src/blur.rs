use bevy::{
    core_pipeline::{core_2d::graph::Node2d, fullscreen_material::FullscreenMaterial},
    prelude::*,
    render::{
        extract_component::ExtractComponent, render_graph::RenderLabel, render_resource::ShaderType,
    },
};

#[derive(Component, ExtractComponent, Clone, Copy, ShaderType)]
pub struct BlurEffect {
    strength: f32,
    darkness: f32,
}

impl Default for BlurEffect {
    fn default() -> Self {
        Self {
            strength: 1.8,
            darkness: 0.3,
        }
    }
}

impl FullscreenMaterial for BlurEffect {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "shader/blur.wgsl".into()
    }

    fn node_edges() -> Vec<bevy::render::render_graph::InternedRenderLabel> {
        vec![
            Node2d::Tonemapping.intern(),
            // post-processing
            Self::node_label().intern(),
            Node2d::EndMainPassPostProcessing.intern(),
        ]
    }
}
