use bevy::{prelude::*, DefaultPlugins};
use bevy_flycam::prelude::*;

mod terrain;

use bevy_mod_picking::{PickingCameraBundle, PickableBundle, DefaultPickingPlugins, DebugCursorPickingPlugin};
use terrain::*;
use terrain::bundle::TerrainBundle;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerrainPlugin)
        .add_plugin(NoCameraPlayerPlugin)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(DebugCursorPickingPlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(terrain_test)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        FlyCam,
        PickingCameraBundle::default(),
    ));
}

fn terrain_test(
    mut commands: Commands,
    mut terrains: ResMut<Assets<Terrain>>,
    mut materials: ResMut<Assets<TerrainMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let handle: Handle<Image> = asset_server.load("images/Heightmap.png");
    let terrain = Terrain::new(Vec3::new(20.0, 2.0, 20.0), handle);
    let terrain_handle = terrains.add(terrain);

    commands.spawn((
        TerrainBundle {
            terrain: terrain_handle,
            material: materials.add(TerrainMaterial { color: Color::WHITE }),
            ..Default::default()
        },
        PickableBundle::default(),
    ));
}