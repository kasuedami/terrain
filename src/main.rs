use bevy::{prelude::*, DefaultPlugins};
use bevy_flycam::prelude::*;

mod terrain;

use terrain::*;
use terrain::bundle::TerrainBundle;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerrainPlugin)
        // .add_startup_system(camera_setup)
        .add_startup_system(terrain_test)
        .add_plugin(PlayerPlugin)
        .run();
}

fn terrain_test(
    mut commands: Commands,
    mut terrains: ResMut<Assets<Terrain>>,
    mut materials: ResMut<Assets<TerrainMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let handle: Handle<Image> = asset_server.load("images/small.png");
    let terrain = Terrain::new(
        "Tester".to_owned(),
        Vec3::new(2.0, 0.5, 2.0),
        handle,
    );

    let terrain_handle = terrains.add(terrain);

    commands.spawn(TerrainBundle {
        terrain: terrain_handle,
        material: materials.add(TerrainMaterial { color: Color::WHITE }),
        ..Default::default()
    });
}