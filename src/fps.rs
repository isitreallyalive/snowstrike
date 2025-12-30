use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use snowstrike::Layers;

pub fn plugin(app: &mut App) {
    app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(Startup, spawn)
            .add_systems(Update, update);
}

#[derive(Component)]
struct Fps;

fn spawn(mut commands: Commands, server: Res<AssetServer>) {
    let font = server.load("fonts/CountingApples.ttf");

    commands.spawn((
        Text::new("--"),
        TextFont {
            font,
            font_size: 4.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(3.0),
            right: Val::Px(3.0),
            ..default()
        },
        Fps,
        Layers::UI,
    ));
}

fn update(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<Fps>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            **text = format!("{fps:.0}");
        }
    }
}