use bevy::{prelude::{Plugin, Component, IVec3, Color, Handle, Image, Material, Mesh, MaterialPlugin}, render::{render_resource::{AsBindGroup, VertexFormat}, mesh::MeshVertexAttribute}, reflect::TypeUuid};

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(MaterialPlugin::<TerrainMaterial>::default());
    }
}

#[derive(Component)]
pub struct Terrain {
    size: IVec3,
    heightmap: Handle<Image>,
    shade: Color,
}

pub const ATTRIBUTE_SHADE_COLOR: MeshVertexAttribute =
    MeshVertexAttribute::new("ShadeColor", 229103874, VertexFormat::Float32x4);

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "1f95edd0-6749-40bd-8e03-b05c5bf948ff"]
pub struct TerrainMaterial {
    #[uniform(0)]
    pub color: Color,
}

impl Material for TerrainMaterial {
    fn vertex_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/terrain_shader.wgsl".into()
    }

    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/terrain_shader.wgsl".into()
    }

    fn specialize(
            _pipeline: &bevy::pbr::MaterialPipeline<Self>,
            descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
            layout: &bevy::render::mesh::MeshVertexBufferLayout,
            _key: bevy::pbr::MaterialPipelineKey<Self>,
        ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        let vertex_layout = layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            ATTRIBUTE_SHADE_COLOR.at_shader_location(1),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}