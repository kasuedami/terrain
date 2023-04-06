use bevy::{prelude::*, DefaultPlugins};

mod terrain;

use terrain::*;
use terrain::bundle::TerrainBundle;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerrainPlugin)
        .add_startup_system(camera_setup)
        .add_startup_system(terrain_test)
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
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
        IVec3::new(2, 1, 2),
        handle,
        Color::RED);

    let terrain_handle = terrains.add(terrain);

    commands.spawn(TerrainBundle {
        terrain: terrain_handle,
        material: materials.add(TerrainMaterial { color: Color::WHITE }),
        ..Default::default()
    });
}