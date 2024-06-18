mod base;
mod clock;
mod input_node;
mod wire;

use bevy::prelude::*;
use clock::ClockPlugin;
use input_node::InputNodePlugin;
use wire::WirePlugin;

fn main() {
    App::new()
        // bevy built-in plugins
        .add_plugins(DefaultPlugins)
        // custom plugins
        .add_plugins(ClockPlugin)
        .add_plugins(InputNodePlugin)
        .add_plugins(WirePlugin)
        .run();
}
