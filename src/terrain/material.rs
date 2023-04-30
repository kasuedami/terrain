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
pub struct TerrainMaterial {
    #[texture(0)]
    #[sampler(1)]
    atlas: Handle<Image>,
    #[texture(2)]
    first: Option<Handle<Image>>,
    #[texture(3)]
    second: Option<Handle<Image>>,
    #[texture(4)]
    third: Option<Handle<Image>>,
    #[texture(5)]
    fourth: Option<Handle<Image>>,
}

impl TerrainMaterial {
    pub fn new(
        atlas: Handle<Image>,
        first: Option<Handle<Image>>,
        second: Option<Handle<Image>>,
        third: Option<Handle<Image>>,
        fourth: Option<Handle<Image>>
    ) -> Self {
        TerrainMaterial {
            atlas,
            first,
            second,
            third,
            fourth
        }
    }
}

impl Material for TerrainMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/terrain_shader.wgsl".into()
    }
}