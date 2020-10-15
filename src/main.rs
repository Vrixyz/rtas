use bevy::prelude::*;

mod core_game;
mod client;

use core_game::CorePlugin;
use client::ClientPlugin;

fn main() {
    App::build()
    .add_default_plugins()
    .add_plugin(ClientPlugin)
    .add_plugin(CorePlugin)
    .run();
}
