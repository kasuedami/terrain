@group(1) @binding(0)
var atlasTexture: texture_2d<f32>;
@group(1) @binding(1)
var textureSampler: sampler;
@group(1) @binding(2)
var firstTexture: texture_2d<f32>;
@group(1) @binding(3)
var secondTexture: texture_2d<f32>;
@group(1) @binding(4)
var thirdTexture: texture_2d<f32>;

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var atlas_color = textureSample(atlasTexture, textureSampler, in.uv);

    var first_color = textureSample(firstTexture, textureSampler, in.uv) * atlas_color.x;
    var second_color = textureSample(secondTexture, textureSampler, in.uv) * atlas_color.y;
    var third_color = textureSample(secondTexture, textureSampler, in.uv) * atlas_color.z;

    return first_color + second_color + third_color;
}