use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_discord_rpc::{Activity, Timestamps};
use snowstrike::GameState;

use crate::menu::button::TextureButton;

mod button;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Menu), main_menu)
        .add_systems(Update, button::process.run_if(in_state(GameState::Menu)));
}

fn main_menu(mut commands: Commands, assets: Res<AssetServer>, mut activity: ResMut<Activity>) {
    // set main menu activity
    activity.update(|a| {
        a.details = Some("On ice".to_string());
        a.state = Some("In the main menus".to_string());
        a.timestamps = Timestamps::now().ok();
    });

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
    ));

    // play button
    let play_button = assets.load("menu/play.aseprite");

    commands.spawn((
        TextureButton::from(play_button),
        Node {
            margin: UiRect::horizontal(Val::Auto).with_top(Val::Vh(30.)),
            ..default()
        },
    ));

    // version
    let font = assets.load("fonts/CountingApples.ttf");
    commands.spawn((
        Text::new(format!("snowstrike v{}", env!("CARGO_PKG_VERSION"))),
        TextFont {
            font,
            font_size: 4.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(Justify::Right),
        Node {
            position_type: PositionType::Absolute,
            right: Val::Vw(1.0),
            bottom: Val::Vh(1.0),
            ..default()
        },
    ));
}
