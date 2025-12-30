use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use snowstrike::{GameState, Layers};

use crate::menu::button::TextureButton;

mod button;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Menu), main_menu)
        .add_systems(Update, button::process.run_if(in_state(GameState::Menu)));
}

fn main_menu(mut commands: Commands, assets: Res<AssetServer>) {
    // title
    commands.spawn((
        AseAnimation {
            aseprite: assets.load("menu/title.aseprite"),
            ..default()
        },
        ImageNode::default(),
        Node {
            margin: UiRect::horizontal(Val::Auto).with_top(Val::Vh(10.)),
            ..default()
        },
        Layers::UI,
    ));

    // play button
    let play_button = assets.load("menu/play.aseprite");

    commands.spawn((
        TextureButton::from(play_button),
        Node {
            margin: UiRect::horizontal(Val::Auto).with_top(Val::Vh(30.)),
            ..default()
        }
    ));
}