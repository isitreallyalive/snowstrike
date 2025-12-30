use bevy::{prelude::*, sprite_render::Material2dPlugin};
use bevy_aseprite_ultra::prelude::*;
use bevy_modern_pixel_camera::prelude::*;
use snowstrike::GameState;

mod blur;
mod menu;
mod setup;

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
        .add_systems(Startup, (setup::icon, setup::camera, setup::draw_map))
        .add_plugins((
            Material2dPlugin::<blur::BlurMaterial>::default(),
            menu::plugin,
        ))
        .init_state::<GameState>()
        .run();
}
