use bevy::{prelude::*};
use bevy_aseprite_ultra::prelude::*;

#[derive(Bundle, Default)]
pub struct Player {
    aseprite: AseAnimation,
    animation: Animation,
    sprite: Sprite,
}

#[derive(Component)]
pub enum Animation {
    Idle(Facing),
    Run(Facing),
}

impl Default for Animation {
    fn default() -> Self {
        Animation::Idle(Facing::default())
    }
}

impl Animation {
    const fn tag(&self) -> &'static str {
        match self {
            Animation::Idle(facing) => match facing {
                Facing::North => "IdleN",
                Facing::East => "IdleE",
                Facing::South => "IdleS",
                Facing::West => "IdleW",
            },
            Animation::Run(facing) => match facing {
                Facing::North => "RunN",
                Facing::East => "RunE",
                Facing::South => "RunS",
                Facing::West => "RunW",
            },
        }
    }
}

#[derive(Default)]
pub enum Facing {
    North,
    East,
    #[default]
    South,
    West,
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn);
}

pub fn spawn(mut commands: Commands, server: Res<AssetServer>) {
    let animation = Animation::default();

    commands.spawn(Player {
        aseprite: AseAnimation {
            aseprite: server.load("sprites/penguin.aseprite"),
            animation: animation.tag().into(),
        },
        animation,
        sprite: Sprite::default(),
    });
}
