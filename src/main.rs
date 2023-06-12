use bevy::diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::{prelude::*, DefaultPlugins};
use bevy_flycam::prelude::*;
use bevy_mod_picking::{PickingCameraBundle, PickableBundle, DebugCursorPickingPlugin, DebugEventsPickingPlugin, PickingPlugin, InteractablePickingPlugin};

mod terrain;

use terrain::*;
use terrain::bundle::TerrainBundle;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..Default::default()
        }))
        .add_plugin(TerrainPlugin)
        .add_plugin(NoCameraPlayerPlugin)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_plugin(DebugCursorPickingPlugin)
        .add_plugin(DebugEventsPickingPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
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
    asset_server: Res<AssetServer>,
) {
    let loaded_terrain: Handle<Terrain> = asset_server.load("terrain/simple.terrain");

    commands.spawn((
        TerrainBundle {
            terrain: loaded_terrain,
            ..Default::default()
        },
        PickableBundle::default(),
    ));
}