#![feature(trait_alias)]

use bevy::prelude::*;

pub mod button;

pub const MAP_WIDTH: u32 = 207;
pub const MAP_HEIGHT: u32 = 151;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, States)]
pub enum GameState {
    #[default]
    Menu,
}
