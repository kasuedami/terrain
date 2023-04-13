@group(1) @binding(0)
var atlasTexture: texture_2d<f32>;
@group(1) @binding(1)
var textureSampler: sampler;
@group(1) @binding(2)
var firstTexture: texture_2d<f32>;

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var color = vec4(1.0, 0.0, 0.0, 1.0);
    var atlas_color = textureSample(atlasTexture, textureSampler, in.uv);
    color = textureSample(firstTexture, textureSampler, in.uv) * atlas_color.x;

    return color;
}