use bevy::{prelude::*, ui::widget::Button};
use bevy_aseprite_ultra::prelude::*;

#[derive(Bundle)]
pub struct TextureButton {
    animation: AseAnimation,
    image: ImageNode,
    ui: Button,
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
            ui: Button,
        }
    }
}

pub fn process(
    mut query: Query<(&Interaction, &mut AseAnimation), Changed<Interaction>>,
    assets: Res<AssetServer>,
    mut commands: Commands,
) {
    for (interaction, mut animation) in query.iter_mut() {
        let tag = match *interaction {
            Interaction::Pressed => "Click",
            Interaction::Hovered => "Hover",
            Interaction::None => "Idle",
        };
        animation.animation = Animation::tag(tag);

        if Interaction::Pressed == *interaction {
            let click = assets.load("sfx/click1.ogg");
            commands.spawn(AudioPlayer::new(click));
        }
    }
}
