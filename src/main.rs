mod board;
mod environment;
mod pieces;

use bevy::prelude::*;
use bevy_mod_picking::{DebugPickingPlugin, PickingPlugin};

fn main() {
    App::build()
        // Set antialiasing to use 4 samples
        .add_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .add_resource(WindowDescriptor {
            title: "Chess!".to_string(),
            width: 960.,
            height: 540.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PickingPlugin)
        .add_plugin(DebugPickingPlugin)
        .add_plugin(board::BoardPlugin)
        .add_startup_system(environment::setup.system())
        .add_startup_system(pieces::create.system())
        .run();
}
