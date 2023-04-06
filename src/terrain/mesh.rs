use super::{Terrain, ATTRIBUTE_SHADE_COLOR};

use bevy::prelude::*;
use bevy::render::mesh::{self, PrimitiveTopology};

pub fn generate_mesh(mut terrain: &mut Terrain, meshes: &mut ResMut<Assets<Mesh>>, images: &Res<Assets<Image>>) {
    let current_mesh = meshes.get(&terrain.mesh);
    let new_mesh = caluclate_mesh(terrain, images);

    if current_mesh.is_some() {
        let current_mesh = meshes.get_mut(&terrain.mesh).unwrap();
        *current_mesh = new_mesh;

    } else {
        let mesh_handle = meshes.add(new_mesh);
        terrain.mesh = mesh_handle;
    }
}

pub fn regenerate_mesh(terrain: &Terrain, meshes: &mut ResMut<Assets<Mesh>>, images: &Res<Assets<Image>>) {
    let current_mesh = meshes.get_mut(&terrain.mesh);
    
    if let Some(current_mesh) = current_mesh {
        *current_mesh = caluclate_mesh(terrain, images);
    }
}

fn caluclate_mesh(terrain: &Terrain, images: &Res<Assets<Image>>) -> Mesh {
    
    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    let total_width = terrain.size.x;
    let total_length = terrain.size.z;
    let half_width = total_width / 2.0;
    let half_length = total_length / 2.0;
    let max_height = terrain.size.y;

    let heightmap = images.get(&terrain.heightmap).unwrap();
    let width_resolution = heightmap.size().x as usize;
    let lengh_resolution = heightmap.size().y as usize;
    
    let mut positions = Vec::with_capacity(width_resolution * lengh_resolution);
    let mut shade_colors = Vec::with_capacity(width_resolution * lengh_resolution);

    for y in 0..lengh_resolution {
        for x in 0..width_resolution {
            // currently only for 4 channel
            let point_index = y * width_resolution + x;
            let raw_height = heightmap.data[point_index * 4];
            let normalized_height = raw_height as f32 / 255.0;
            let height = normalized_height * max_height;
            
            let width_position = (x as f32 / width_resolution as f32) * total_width - half_width;
            let length_position = (y as f32 / lengh_resolution as f32) * total_length - half_length;

            positions.push([width_position, height, length_position]);

            let lower_hue = (x as f32 / width_resolution as f32) * 18.0 + 1.0;
            let upper_hue = (y as f32 / lengh_resolution as f32) * 18.0 + 1.0;
            let total_hue = lower_hue * upper_hue;
            let color = Color::Hsla { hue: total_hue, saturation: 1.0, lightness: 0.5, alpha: 1.0 };
            
            shade_colors.push(color.as_rgba_f32());
        }
    }

    let mut line_indices = Vec::with_capacity(positions.len() * 2);

    for i in 0..(positions.len() as u32) - 1 {
        line_indices.push(i);
        line_indices.push(i + 1);
    }

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION, 
        positions,
    );
    
    mesh.insert_attribute(
        ATTRIBUTE_SHADE_COLOR, 
        shade_colors,
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![[0.0, 1.0, 0.0]; width_resolution * lengh_resolution],
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0.0, 0.0]; width_resolution * lengh_resolution],
    );

    mesh.set_indices(Some(mesh::Indices::U32(
        line_indices
    )));
    
    mesh
}

#[allow(dead_code)]
fn caluclate_flat_mesh(terrain: &Terrain) -> Mesh {
    
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let total_width = terrain.size.x as f32;
    let total_length = terrain.size.z as f32;
    let half_width = total_width / 2.0;
    let half_length = total_length / 2.0;

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION, 
        vec![
            [-half_width, 0.0, -half_length],
            [ half_width, 0.0, -half_length],
            [-half_width, 0.0,  half_length],
            [ half_width, 0.0,  half_length],
        ],
    );
    
    mesh.insert_attribute(
        ATTRIBUTE_SHADE_COLOR, 
        vec![
            Color::rgb(1.0, 0.0, 0.0).as_rgba_f32(),
            Color::rgb(0.0, 1.0, 0.0).as_rgba_f32(),
            Color::rgb(0.0, 0.0, 1.0).as_rgba_f32(),
            Color::rgb(0.0, 0.0, 0.0).as_rgba_f32(),
        ],
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![[0.0, 1.0, 0.0]; 4],
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0.0, 0.0]; 4],
    );
    
    mesh.set_indices(Some(mesh::Indices::U32(
        vec![0, 2, 1, 2, 3, 1]
    )));
    
    mesh
}