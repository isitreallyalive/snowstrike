use bevy::{diagnostic::FrameCount, prelude::*};
use bevy_aseprite_ultra::prelude::*;
use bevy_modern_pixel_camera::prelude::*;

use crate::blur::BlurEffect;

const MAP_ZOOM: PixelZoom = PixelZoom::FitSize {
    width: 207,
    height: 151,
};

/// Make the window visible after a few frames have passed to avoid seeing a white flash.
pub fn make_visible(mut window: Single<&mut Window>, frames: Res<FrameCount>) {
    if frames.0 == 3 {
        window.visible = true;
    }
}

/// Spawn the main camera and draw the map background.
pub fn camera(mut commands: Commands, server: Res<AssetServer>) {
    // spawn camera
    commands.spawn((
        Camera2d,
        Msaa::Off,
        MAP_ZOOM,
        PixelViewport,
        WithUiScaling,
        BlurEffect::default(),
    ));

    // draw map
    commands.spawn((
        AseAnimation {
            aseprite: server.load("map/map.aseprite"),
            ..default()
        },
        Sprite::default(),
    ));
}
