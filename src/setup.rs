use bevy::{diagnostic::FrameCount, prelude::*};
use bevy_aseprite_ultra::prelude::*;
use bevy_modern_pixel_camera::prelude::*;
use snowstrike::{MAP_HEIGHT, MAP_WIDTH};

use crate::blur::BlurEffect;

pub fn make_visible(mut window: Single<&mut Window>, frames: Res<FrameCount>) {
    // try to hide the white!
    if frames.0 == 3 {
        window.visible = true;
    }
}

pub fn camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Msaa::Off,
        PixelZoom::FitSize {
            width: MAP_WIDTH as i32,
            height: MAP_HEIGHT as i32,
        },
        PixelViewport,
        WithUiScaling,
        BlurEffect::default()
    ));
}

/// Draw the map in the background
pub fn draw_map(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((
        AseAnimation {
            aseprite: server.load("map/map.aseprite"),
            ..default()
        },
        Sprite::default(),
    ));
}
