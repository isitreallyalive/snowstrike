// todo: embed assets
// todo: asset processor

use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_discord_rpc::prelude::*;
use bevy_icon::prelude::*;
use bevy_modern_pixel_camera::prelude::*;

/// Discord application client ID for RPC
const DISCORD_CLIENT_ID: u64 = 1454285370962219008;

/// Application icon data
const ICON_DATA: &[u8] = include_bytes!("../assets/icon.png");

mod blur;
mod fps;
mod menus;
mod setup;

fn main() -> Result<()> {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "snowstrike".to_string(),
                        present_mode: bevy::window::PresentMode::AutoNoVsync,
                        visible: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(DiscordRpcPlugin::builder(DISCORD_CLIENT_ID).build())
        .add_plugins(BevyIconPlugin::new(Icon::from(image::load_from_memory(
            ICON_DATA,
        )?)))
        .add_plugins((PixelCameraPlugin, AsepriteUltraPlugin))
        .add_systems(Startup, setup::camera)
        .add_systems(Update, setup::make_visible)
        .add_plugins((fps::plugin, menus::plugin, blur::plugin))
        .init_asset::<AudioSource>()
        .init_state::<snowstrike::GameState>()
        .run();

    Ok(())
}
