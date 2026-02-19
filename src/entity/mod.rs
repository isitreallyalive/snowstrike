use bevy::prelude::*;
use strum::Display;

mod player;

#[derive(Component, Default, Display)]
pub enum Facing {
    #[strum(to_string = "N")]
    North,
    #[strum(to_string = "E")]
    East,
    #[strum(to_string = "S")]
    #[default]
    South,
    #[strum(to_string = "W")]
    West,
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, player::spawn);
}