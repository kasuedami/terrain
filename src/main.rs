use bevy::diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::{prelude::*, DefaultPlugins};
use bevy_flycam::prelude::*;

mod terrain;

use bevy_mod_picking::{PickingCameraBundle, PickableBundle, DebugCursorPickingPlugin, DebugEventsPickingPlugin, PickingPlugin, InteractablePickingPlugin};
use terrain::*;
use terrain::bundle::TerrainBundle;
use terrain::material::{TerrainMaterial, TerrainLayer};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
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
    mut terrains: ResMut<Assets<Terrain>>,
    mut materials: ResMut<Assets<TerrainMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let handle: Handle<Image> = asset_server.load("images/Heightmap.png");
    let terrain = Terrain::new(Vec3::new(20.0, 2.0, 20.0), handle);
    let terrain_handle = terrains.add(terrain);

    let atlas: Handle<Image> = asset_server.load("images/atlas.png");
    let grass: Handle<Image> = asset_server.load("images/grass.jpeg");
    let stone_light: Handle<Image> = asset_server.load("images/stone_light.jpeg");
    let stone_dark: Handle<Image> = asset_server.load("images/stone_dark.jpeg");

    let layers = vec![
        TerrainLayer::new(grass, Vec2::ONE),
        TerrainLayer::new(stone_light, Vec2::ONE),
        TerrainLayer::new(stone_dark, Vec2::ONE),
    ];

    let terrain_material = TerrainMaterial::new(atlas, &layers);

    commands.spawn((
        TerrainBundle {
            terrain: terrain_handle,
            material: materials.add(terrain_material),
            ..Default::default()
        },
        PickableBundle::default(),
    ));
}