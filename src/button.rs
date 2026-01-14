use std::marker::PhantomData;

use bevy::{prelude::*, ui::widget::Button as UiButton};
use bevy_aseprite_ultra::prelude::*;

pub trait Button = Component + Message + Clone + Copy;

#[derive(Bundle)]
pub struct TextureButton<B: Button> {
    animation: AseAnimation,
    image: ImageNode,
    ui: UiButton,
    button: B,
}

impl<B: Button> TextureButton<B> {
    pub fn new(animation: Handle<Aseprite>, button: B) -> Self {
        Self {
            animation: AseAnimation {
                aseprite: animation,
                animation: Animation::tag("Idle"),
                ..default()
            },
            image: ImageNode::default(),
            ui: UiButton,
            button,
        }
    }
}

fn animate<B: Button>(
    mut query: Query<(&Interaction, &mut AseAnimation, &B), Changed<Interaction>>,
    assets: Res<AssetServer>,
    mut commands: Commands,
    mut writer: MessageWriter<B>,
) {
    for (interaction, mut animation, button) in query.iter_mut() {
        let tag = match *interaction {
            Interaction::Pressed => "Click",
            Interaction::Hovered => "Hover",
            Interaction::None => "Idle",
        };
        animation.animation = Animation::tag(tag);

        if Interaction::Pressed == *interaction {
            let click = assets.load("sfx/click1.ogg");
            commands.spawn(AudioPlayer::new(click));
            writer.write(*button);
        }
    }
}

pub struct ButtonPlugin<B: Button, S: States + Copy> {
    state: S,
    _marker: PhantomData<B>,
}

impl<B: Button, S: States + Copy> ButtonPlugin<B, S> {
    pub fn new(state: S) -> Self {
        Self {
            state,
            _marker: PhantomData,
        }
    }
}

impl<B: Button, S: States + Copy> Plugin for ButtonPlugin<B, S> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate::<B>.run_if(in_state(self.state.clone()))).add_message::<B>();
    }
}
