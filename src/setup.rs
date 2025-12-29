use bevy::{ecs::system::NonSendMarker, prelude::*, winit::{WINIT_WINDOWS, WinitWindows}};
use bevy_modern_pixel_camera::prelude::*;
use image::{GenericImageView, ImageFormat};
use snowstrike::{MAP_HEIGHT, MAP_WIDTH};
use winit::window::Icon;

const ICON_DATA: &[u8] = include_bytes!("../assets/icon.png");

/// Set the window's icon
pub fn icon(_: NonSendMarker) -> Result<()> {
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
        if windows.is_empty() { return; }
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