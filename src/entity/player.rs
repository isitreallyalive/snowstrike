use bevy::{prelude::*};
use bevy_aseprite_ultra::prelude::*;
use strum::Display;

use crate::entity::Facing;

#[derive(Bundle, Default)]
pub struct Player {
    aseprite: AseAnimation,
    animation: Animation,
    facing: Facing,
    sprite: Sprite,
}

#[derive(Component, Default, Display)]
pub enum Animation {
    #[default]
    Idle,
    Run
}

pub fn spawn(mut commands: Commands, server: Res<AssetServer>) {
    let animation = Animation::default();
    let facing = Facing::default();

    commands.spawn(Player {
        aseprite: AseAnimation {
            aseprite: server.load("sprites/penguin.aseprite"),
            animation: format!("{animation}{facing}").as_str().into(),
        },
        animation,
        facing,
        sprite: Sprite::default(),
    });
}
