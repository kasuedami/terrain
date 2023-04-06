use super::{Terrain, ATTRIBUTE_SHADE_COLOR};

use bevy::prelude::*;
use bevy::render::mesh::{self, PrimitiveTopology};

pub fn generate_mesh(mut terrain: &mut Terrain, meshes: &mut ResMut<Assets<Mesh>>) {
    let current_mesh = meshes.get(&terrain.mesh);
    let new_mesh = caluclate_mesh(terrain);

    if current_mesh.is_some() {
        let current_mesh = meshes.get_mut(&terrain.mesh).unwrap();
        *current_mesh = new_mesh;

    } else {
        let mesh_handle = meshes.add(new_mesh);
        terrain.mesh = mesh_handle;
    }
}

pub fn regenerate_mesh(terrain: &Terrain, meshes: &mut ResMut<Assets<Mesh>>) {
    let current_mesh = meshes.get_mut(&terrain.mesh);
    
    if let Some(current_mesh) = current_mesh {
        *current_mesh = caluclate_mesh(terrain);
    }
}

fn caluclate_mesh(terrain: &Terrain) -> Mesh {
    
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let half_width = terrain.size.x as f32 / 2.0;
    let half_length = terrain.size.y as f32 / 2.0;

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