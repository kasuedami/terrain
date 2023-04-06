use bevy::{prelude::*, DefaultPlugins};

mod terrain;

use terrain::*;
use terrain::bundle::TerrainBundle;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerrainPlugin)
        .add_startup_system(camera_setup)
        // .add_startup_system(setup)
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
    let handle: Handle<Image> = asset_server.load("images/Heightmap.png");
    let terrain = Terrain::new(
        "Tester".to_owned(),
        IVec3::new(10, 10, 10),
        handle,
        Color::RED);

    let terrain_handle = terrains.add(terrain);

    commands.spawn(TerrainBundle {
        terrain: terrain_handle,
        material: materials.add(TerrainMaterial { color: Color::WHITE }),
        ..Default::default()
    });
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TerrainMaterial>>
) {
    let mut mesh = Mesh::from(shape::Cube { size: 1.0 });
    
    let mesh_vertex_count = 24;
    let attribute_color_values: Vec<_> = (0..mesh_vertex_count).into_iter().map(|i| {
        let hue = i as f32 / mesh_vertex_count as f32 * 360.0;
        let color = Color::Hsla { hue, saturation: 0.5, lightness: 0.5, alpha: 1.0 };
        color.as_rgba_f32()
    }).collect();

    dbg!(&attribute_color_values);

    mesh.insert_attribute(terrain::ATTRIBUTE_SHADE_COLOR, attribute_color_values);
    
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(mesh),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: materials.add(terrain::TerrainMaterial {
            color: Color::WHITE,
        }),
        ..default()
    });

}
