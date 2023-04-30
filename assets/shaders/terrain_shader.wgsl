@group(1) @binding(0)
var nearest_sampler: sampler;
@group(1) @binding(1)
var atlas_texture: texture_2d<f32>;
@group(1) @binding(2)
var textures: binding_array<texture_2d<f32>>;

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var atlas_color = textureSample(atlas_texture, nearest_sampler, in.uv);

    var first_color = textureSample(textures[0], nearest_sampler, in.uv) * atlas_color.x;
    var second_color = textureSample(textures[1], nearest_sampler, in.uv) * atlas_color.y;
    var third_color = textureSample(textures[2], nearest_sampler, in.uv) * atlas_color.z;

    return first_color + second_color + third_color;
}