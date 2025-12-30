use bevy::{camera::visibility::RenderLayers, prelude::*, ui::widget::Button};
use bevy_aseprite_ultra::prelude::*;
use snowstrike::Layers;

#[derive(Bundle)]
pub struct TextureButton {
    animation: AseAnimation,
    image: ImageNode,
    layer: RenderLayers,
    ui: Button
}

impl From<Handle<Aseprite>> for TextureButton {
    fn from(aseprite: Handle<Aseprite>) -> Self {
        TextureButton {
            animation: AseAnimation {
                aseprite,
                animation: Animation::tag("Idle"),
                ..default()
            },
            image: ImageNode::default(),
            layer: Layers::UI,
            ui: Button
        }
    }
}

pub fn animate(mut query: Query<(&Interaction, &mut AseAnimation), Changed<Interaction>>) {
    for (interaction, mut animation) in query.iter_mut() {
        let tag = match *interaction {
            Interaction::Pressed => "Click",
            Interaction::Hovered => "Hover",
            Interaction::None => "Idle",
        };
        animation.animation = Animation::tag(tag);
    }
}
