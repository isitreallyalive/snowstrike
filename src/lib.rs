#![feature(trait_alias)]

use bevy::prelude::*;

pub mod ui;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, States)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
}
