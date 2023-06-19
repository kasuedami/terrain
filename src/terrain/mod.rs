use bevy::{prelude::*, reflect::{TypeUuid, TypePath}, render::render_resource::{AsBindGroup, ShaderRef, ShaderType}};

mod editor;
mod loader;
mod mesh;

pub mod bundle;
pub mod plugin;

#[derive(Asset, TypePath, AsBindGroup, Debug, TypeUuid, Clone)]
#[uuid = "ee330faa-acb4-45b9-9309-c272f1438d7e"]
pub struct Terrain {
    size: Vec3,
    heightmap: Handle<Image>,
    #[sampler(0)]
    #[texture(1)]    
    atlas: Handle<Image>,
    #[texture(2)]
    red_texture: Option<Handle<Image>>,
    #[storage(3, read_only)]
    red_layer: Layer,
    #[texture(4)]
    green_texture: Option<Handle<Image>>,
    #[storage(5, read_only)]
    green_layer: Layer,
    #[texture(6)]
    blue_texture: Option<Handle<Image>>,
    #[storage(7, read_only)]
    blue_layer: Layer,
    mesh: Handle<Mesh>,
}

impl Terrain {
    pub fn new(
        size: Vec3,
        heightmap: Handle<Image>,
        atlas: Handle<Image>,
        red_texture: Option<Handle<Image>>,
        red_layer: Layer,
        green_texture: Option<Handle<Image>>,
        green_layer: Layer,
        blue_texture: Option<Handle<Image>>,
        blue_layer: Layer,
        mesh: Handle<Mesh>,
    ) -> Self {
        Terrain {
            size,
            heightmap,
            atlas,
            red_texture,
            red_layer,
            green_texture,
            green_layer,
            blue_texture,
            blue_layer,
            mesh,
        }
    }
}

impl Material for Terrain {
    fn fragment_shader() -> ShaderRef {
        "shaders/terrain_shader.wgsl".into()
    }
}

#[derive(Debug, Clone, ShaderType)]
pub struct Layer {
    scaling: Vec2,
}

impl Layer {
    pub fn new(scaling: Vec2) -> Self {
        Layer {
            scaling
        }
    }
}

impl Default for Layer {
    fn default() -> Self {
        Self { scaling: Vec2::ONE }
    }
}