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
    // extract some values from the parameters for better readabillity
    let total_width = terrain.size.x;
    let total_length = terrain.size.z;
    let half_width = total_width / 2.0;
    let half_length = total_length / 2.0;
    let max_height = terrain.size.y;

    // retreive the heightmap image and extract its size
    let heightmap = images.get(&terrain.heightmap).unwrap();
    let width_resolution = heightmap.size().x as usize;
    let length_resolution = heightmap.size().y as usize;
    
    // calculate the ammount of vertices for the mesh
    let vertices_count = width_resolution * length_resolution;

    // initialize the positions and shade_colors vectors with
    // capacity of verticies_count which leads to no additional
    // allocations during mesh construction
    let mut positions = Vec::with_capacity(vertices_count);
    let mut shade_colors = Vec::with_capacity(vertices_count);

    for y in 0..length_resolution {
        for x in 0..width_resolution {
            // calculate the index of the pixel for the linear data vector
            let pixel_index = y * width_resolution + x;
            // retreive the raw height data
            // currently only images using 4 channels per pixel
            // are supported but only the red channel is used
            // this leads to the * 4 multiplication of the pixel_index
            let raw_height = heightmap.data[pixel_index * 4];
            // the raw_height is normalized between 0 and 1 as
            // its maximal possible value is 255
            let normalized_height = raw_height as f32 / 255.0;
            // calculate the actual height by multiplying with max_height
            let height = normalized_height * max_height;
            
            // calculate 'percentage of loop variable in their resolution
            // before scaling with the total size and offseting them by half
            let width_position = (x as f32 / width_resolution as f32) * total_width - half_width;
            let length_position = (y as f32 / length_resolution as f32) * total_length - half_length;

            // push the calculated position into the positions vector
            positions.push([width_position, height, length_position]);

            // approximate a hue gradient traversing the terrain diagonally
            let lower_hue = (x as f32 / width_resolution as f32) * 18.0 + 1.0;
            let upper_hue = (y as f32 / length_resolution as f32) * 18.0 + 1.0;
            let total_hue = lower_hue * upper_hue;
            let color = Color::Hsla { hue: total_hue, saturation: 0.8, lightness: 0.3, alpha: 1.0 };
            
            // push the calculated color into the shade_colors vector
            shade_colors.push(color.as_rgba_f32());
        }
    }

    // calculate the count of 'rectangles' and triangles that the terrain consists of
    let rectangle_count = (length_resolution - 1) * (width_resolution - 1);
    let triangle_count = rectangle_count * 2;
    
    // initialize indicies with an initial capacity of triangle_count * 3
    // as each triangle consists of three points
    let mut indices = Vec::with_capacity(triangle_count * 3);

    for l in 0..(length_resolution as u32) - 1 {
        for w in 0..(width_resolution as u32) - 1 {
            // the first two indices are in the 'current' line
            let first = l * (width_resolution as u32) + w;
            let second = first + 1;
            // the last two indices are in the 'next' line
            let thrid = (l + 1) * (width_resolution as u32) + w;
            let fourth = thrid + 1;

            // first triangle
            indices.push(first);
            indices.push(thrid);
            indices.push(second);
            
            // second triangle
            indices.push(thrid);
            indices.push(fourth);
            indices.push(second);
        }
    }
    
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    
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
        vec![[0.0, 1.0, 0.0]; width_resolution * length_resolution],
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0.0, 0.0]; width_resolution * length_resolution],
    );

    mesh.set_indices(Some(mesh::Indices::U32(
        indices
    )));
    
    mesh
}