use bevy::prelude::*;

mod client;
mod core_game;

use client::ClientPlugin;
use core_game::CorePlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(ClientPlugin)
        .add_plugin(CorePlugin)
        .run();
}
