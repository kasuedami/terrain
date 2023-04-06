use super::Terrain;

use bevy::prelude::{*, shape::{Plane, Cube}};

pub fn generate_mesh(mut terrain: &mut Terrain, meshes: &mut ResMut<Assets<Mesh>>) {
    // let mesh = Mesh::from(Plane::from_size(terrain.size.x as f32));
    let mut mesh = Mesh::from(shape::Cube { size: 1.0 });
    
    let mesh_vertex_count = 24;
    let attribute_color_values: Vec<_> = (0..mesh_vertex_count).into_iter().map(|i| {
        let hue = i as f32 / mesh_vertex_count as f32 * 360.0;
        let color = Color::Hsla { hue, saturation: 0.5, lightness: 0.5, alpha: 1.0 };
        color.as_rgba_f32()
    }).collect();

    dbg!(&attribute_color_values);

    mesh.insert_attribute(super::ATTRIBUTE_SHADE_COLOR, attribute_color_values);

    let mesh_handle = meshes.add(mesh);
    terrain.mesh = mesh_handle;
}

pub fn regenerate_mesh(terrain: &Terrain, meshes: &mut ResMut<Assets<Mesh>>) {
    // let mesh = Mesh::from(Plane::from_size(terrain.size.x as f32));

    let current_mesh = meshes.get_mut(&terrain.mesh);
    
    if let Some(current_mesh) = current_mesh {
        // current_mesh = Mesh::from(Cube { size: 1.0 });

        let mut mesh = Mesh::from(shape::Cube { size: 1.0 });
    
        let mesh_vertex_count = 24;
        let attribute_color_values: Vec<_> = (0..mesh_vertex_count).into_iter().map(|i| {
            let hue = i as f32 / mesh_vertex_count as f32 * 360.0;
            let color = Color::Hsla { hue, saturation: 0.8, lightness: 0.2, alpha: 1.0 };
            color.as_rgba_f32()
        }).collect();

        mesh.insert_attribute(super::ATTRIBUTE_SHADE_COLOR, attribute_color_values);

        *current_mesh = mesh;
        info!("could access it {:?}", current_mesh);
    }
}