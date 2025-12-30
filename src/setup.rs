use bevy::{
    ecs::system::NonSendMarker,
    prelude::*,
    winit::{WINIT_WINDOWS, WinitWindows},
};
use bevy_aseprite_ultra::prelude::*;
use bevy_modern_pixel_camera::prelude::*;
use image::{GenericImageView, ImageFormat};
use snowstrike::{MAP_HEIGHT, MAP_WIDTH};
use winit::window::Icon;

const ICON_DATA: &[u8] = include_bytes!("../assets/icon.png");

/// Set the window's icon
pub fn icon(_: NonSendMarker, // needs to run on the main thread
) -> Result<()> {
    // load the icon
    let (rgba, width, height) = {
        let image = image::load_from_memory_with_format(ICON_DATA, ImageFormat::Png)?;
        let (width, height) = image.dimensions();
        let rgba = image.to_rgba8().into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(rgba, width, height)?;

    // set the icon for all windows
    WINIT_WINDOWS.with_borrow_mut(|WinitWindows { windows, .. }| {
        if windows.is_empty() {
            return;
        }
        for window in windows.values() {
            window.set_window_icon(Some(icon.clone()));
        }
    });

    Ok(())
}

/// Set up a pixel-perfect camera
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
