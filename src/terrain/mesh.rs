use super::Terrain;

use bevy::prelude::{*, shape::Plane};

pub fn generate_mesh(mut terrain: &mut Terrain, meshes: &mut ResMut<Assets<Mesh>>) {
    let mesh = Mesh::from(Plane::from_size(terrain.size.x as f32));
    let mesh_handle = meshes.add(mesh);
    terrain.mesh = mesh_handle;
}