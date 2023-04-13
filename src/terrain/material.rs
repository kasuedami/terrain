use bevy::{
    prelude::*,
    render::render_resource::{
        AsBindGroup,
        ShaderRef
    },
    reflect::TypeUuid
};

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "f5469063-e5c4-413d-badf-c672caa9147a"]
pub struct TerrainMaterialNew {
    #[texture(0)]
    #[sampler(1)]
    atlas: Handle<Image>,
    #[texture(2)]
    first: Option<Handle<Image>>,
    // #[texture(3)]
    // second: Option<Handle<Image>>,
    // #[texture(4)]
    // third: Option<Handle<Image>>,
    // #[texture(5)]
    // fourth: Option<Handle<Image>>,
}

impl TerrainMaterialNew {
    pub fn new(atlas: Handle<Image>, first: Option<Handle<Image>>) -> Self {
        TerrainMaterialNew { atlas, first }
    }
}

impl Material for TerrainMaterialNew {
    fn fragment_shader() -> ShaderRef {
        "shaders/terrain_shader_new.wgsl".into()
    }
}