use bevy::prelude::*;

pub const MAP_WIDTH: u32 = 207;
pub const MAP_HEIGHT: u32 = 151;

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, States)]
pub enum GameState {
    #[default]
    Menu,
}
