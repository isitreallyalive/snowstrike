use bevy::{
    core_pipeline::{
        core_2d::graph::Node2d,
        fullscreen_material::{FullscreenMaterial, FullscreenMaterialPlugin},
    },
    prelude::*,
    render::{
        extract_component::ExtractComponent, render_graph::RenderLabel, render_resource::ShaderType,
    },
};
use snowstrike::GameState;

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

pub fn plugin(app: &mut App) {
    app.add_plugins(FullscreenMaterialPlugin::<BlurEffect>::default())
        .add_systems(Update, update.run_if(state_changed::<GameState>));
}

/// Blur the camera when not playing
fn update(
    state: Res<State<GameState>>,
    camera: Query<Entity, With<Camera2d>>,
    blur: Query<Entity, With<BlurEffect>>,
    mut commands: Commands,
) {
    if *state == GameState::Playing {
        for entity in blur {
            commands.entity(entity).remove::<BlurEffect>();
        }
    } else {
        for entity in camera.iter() {
            if blur.get(entity).is_err() {
                commands.entity(entity).insert(BlurEffect::default());
            }
        }
    }
}
