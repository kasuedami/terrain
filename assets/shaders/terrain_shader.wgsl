@group(1) @binding(0)
var nearest_sampler: sampler;
@group(1) @binding(1)
var atlas_texture: texture_2d<f32>;
@group(1) @binding(2)
var red_texture: texture_2d<f32>;
@group(1) @binding(3)
var<storage, read> red_layer: TerrainLayer;
@group(1) @binding(4)
var green_texture: texture_2d<f32>;
@group(1) @binding(5)
var<storage, read> green_layer: TerrainLayer;
@group(1) @binding(6)
var blue_texture: texture_2d<f32>;
@group(1) @binding(7)
var<storage, read> blue_layer: TerrainLayer;

struct TerrainLayer {
    scaling: vec2<f32>,
}

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var atlas_color = textureSample(atlas_texture, nearest_sampler, in.uv);

    var red_texture_color = textureSample(red_texture, nearest_sampler, in.uv * red_layer.scaling % vec2(1.0, 1.0)) * atlas_color.x;
    var green_texture_color = textureSample(green_texture, nearest_sampler, in.uv * green_layer.scaling % vec2(1.0, 1.0)) * atlas_color.y;
    var blue_texture_color = textureSample(blue_texture, nearest_sampler, in.uv * blue_layer.scaling % vec2(1.0, 1.0)) * atlas_color.z;
    
    return red_texture_color + green_texture_color + blue_texture_color;
}