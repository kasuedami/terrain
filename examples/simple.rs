use bevy::{prelude::*, DefaultPlugins};

use terrain::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin::default().watch_for_changes()))
        .add_plugin(TerrainPlugin)
        .add_systems(Startup, (setup_camera, terrain_test))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(
        Camera3dBundle {
            transform: Transform::from_xyz(-15.0, 30.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        }
    );
}

fn terrain_test(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let loaded_terrain: Handle<Terrain> = asset_server.load("terrain/simple.terrain");

    commands.spawn((
        TerrainBundle {
            terrain: loaded_terrain,
            ..Default::default()
        },
    ));
}