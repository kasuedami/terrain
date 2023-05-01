use bevy::{
    prelude::*,
    render::render_resource::{
        AsBindGroup,
        ShaderRef, ShaderType,
    },
    reflect::TypeUuid
};

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "cb732d71-3adc-4ebe-b7c1-3e92a7186f29"]
pub struct TerrainMaterial {
    #[sampler(0)]
    #[texture(1)]    
    atlas: Handle<Image>,
    #[texture(2)]
    red_texture: Option<Handle<Image>>,
    #[storage(3, read_only)]
    red_layer: TerrainLayer,
    #[texture(4)]
    green_texture: Option<Handle<Image>>,
    #[storage(5, read_only)]
    green_layer: TerrainLayer,
    #[texture(6)]
    blue_texture: Option<Handle<Image>>,
    #[storage(7, read_only)]
    blue_layer: TerrainLayer,
}

impl TerrainMaterial {
    pub fn new(
        atlas: Handle<Image>,
        red_texture: Option<Handle<Image>>,
        red_layer: Option<TerrainLayer>,
        green_texture: Option<Handle<Image>>,
        green_layer: Option<TerrainLayer>,
        blue_texture: Option<Handle<Image>>,
        blue_layer: Option<TerrainLayer>,
    ) -> Self {
        TerrainMaterial {
            atlas,
            red_texture,
            red_layer: red_layer.unwrap_or_default(),
            green_texture,
            green_layer: green_layer.unwrap_or_default(),
            blue_texture,
            blue_layer: blue_layer.unwrap_or_default(),
        }
    }
}

impl Material for TerrainMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/terrain_shader.wgsl".into()
    }
}

#[derive(Debug, Clone, ShaderType)]
pub struct TerrainLayer {
    scaling: Vec2,
}

impl TerrainLayer {
    pub fn new(scaling: Vec2) -> Self {
        TerrainLayer {
            scaling
        }
    }
}

impl Default for TerrainLayer {
    fn default() -> Self {
        Self { scaling: Vec2::ONE }
    }
}