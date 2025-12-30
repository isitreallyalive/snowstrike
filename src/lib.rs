use bevy::{camera::visibility::RenderLayers, prelude::*};

pub const MAP_WIDTH: u32 = 207;
pub const MAP_HEIGHT: u32 = 151;

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, States)]
pub enum GameState {
    #[default]
    Menu,
}

pub struct Layers;

impl Layers {
    pub const GAME: RenderLayers = RenderLayers::layer(0);
    pub const POST_PROCESS: RenderLayers = RenderLayers::layer(1);
    pub const UI: RenderLayers = RenderLayers::layer(2);
}
