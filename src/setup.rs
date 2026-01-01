use bevy::{
    camera::RenderTarget, diagnostic::FrameCount, ecs::system::NonSendMarker, image::ImageSampler, prelude::*, render::render_resource::TextureFormat, winit::{WINIT_WINDOWS, WinitWindows}
};
use bevy_aseprite_ultra::prelude::*;
use bevy_modern_pixel_camera::prelude::*;
use image::{GenericImageView, ImageFormat};
use snowstrike::{Layers, MAP_HEIGHT, MAP_WIDTH};
use winit::window::Icon;

use crate::blur::BlurMaterial;

const ICON_DATA: &[u8] = include_bytes!("../assets/icon.png");

pub fn make_visible(mut window: Single<&mut Window>, frames: Res<FrameCount>) {
    // try to hide the white!
    if frames.0 == 3 {
        window.visible = true;
    }
}

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

#[derive(Component)]
pub struct PostProcessBlur;

pub fn camera(
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<BlurMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    let pixel_perfect = (
        Camera2d,
        Msaa::Off,
        PixelZoom::FitSize {
            width: MAP_WIDTH as i32,
            height: MAP_HEIGHT as i32,
        },
        PixelViewport,
        WithUiScaling,
    );

    // render all game content to an external texture
    let mut image = Image::new_target_texture(MAP_WIDTH, MAP_HEIGHT, TextureFormat::bevy_default());
    image.sampler = ImageSampler::linear();
    let image_handle = images.add(image);

    commands.spawn((
        pixel_perfect.clone(),
        Camera {
            order: -1, // render first
            target: RenderTarget::from(image_handle.clone()),
            clear_color: Color::WHITE.into(),
            ..default()
        },
        Layers::GAME,
    ));

    // apply a blur post-process effect and render to the main window
    let rect_handle = meshes.add(Rectangle::new(MAP_WIDTH as f32, MAP_HEIGHT as f32));
    let blur_handle = materials.add(BlurMaterial {
        texture: image_handle.clone(),
        strength: BlurMaterial::DEFAULT_STRENGTH,
    });

    commands.spawn((
        Mesh2d(rect_handle),
        MeshMaterial2d(blur_handle),
        Layers::POST_PROCESS,
        PostProcessBlur,
    ));

    commands.spawn((
        pixel_perfect.clone(),
        Camera {
            order: 1,
            ..default()
        },
        Layers::POST_PROCESS,
    ));

    // ui
    commands.spawn((pixel_perfect, Layers::UI));
}

/// Draw the map in the background
pub fn draw_map(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((
        AseAnimation {
            aseprite: server.load("map/map.aseprite"),
            ..default()
        },
        Sprite::default(),
        Layers::GAME,
    ));
}
