use bevy::{prelude::*, DefaultPlugins};

use terrain::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin::default().watch_for_changes()))
        .add_plugin(TerrainPlugin::editor_mode())
        .run();
}