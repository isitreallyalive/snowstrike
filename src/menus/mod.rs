use bevy::prelude::*;

mod main;

pub fn plugin(app: &mut App) {
    app.add_plugins(main::plugin);
}
