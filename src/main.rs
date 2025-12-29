use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_modern_pixel_camera::prelude::*;

const MAP_WIDTH: u32 = 207;
const MAP_HEIGHT: u32 = 151;

fn setup(mut commands: Commands, server: Res<AssetServer>) {
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

    // draw the map in the background
    commands.spawn((
        AseAnimation {
            aseprite: server.load("map.aseprite"),
            ..default()
        },
        Sprite::default(),
    ));
}

fn main() {
    App::new()
        .add_plugins({
            #[allow(unused_mut)]
            let mut default_plugins =
                DefaultPlugins
                    .set(ImagePlugin::default_nearest())
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            title: "snowstrike".to_string(),
                            ..default()
                        }),
                        ..default()
                    });

            // asset processing is only enabled on release builds
            #[cfg(feature = "release")]
            {
                default_plugins = default_plugins.set(AssetPlugin {
                    mode: AssetMode::Processed,
                    ..default()
                });
            }

            default_plugins
        })
        .add_plugins((PixelCameraPlugin, AsepriteUltraPlugin))
        .add_systems(Startup, setup)
        .run();
}
