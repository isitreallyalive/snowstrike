use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use snowstrike::{GameState, Layers};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Menu), draw_title);
}

fn draw_title(mut commands: Commands, assets: Res<AssetServer>) {
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
}
