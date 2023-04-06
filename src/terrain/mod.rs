use std::ops::Deref;

use bevy::{prelude::*, render::{render_resource::{AsBindGroup, VertexFormat}, mesh::MeshVertexAttribute}, reflect::TypeUuid};

pub mod bundle;
mod mesh;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_asset::<Terrain>()
            .add_plugin(MaterialPlugin::<TerrainMaterial>::default())
            .add_system(terrain_mesh_linker);
    }
}

#[derive(Debug, TypeUuid)]
#[uuid = "ee330faa-acb4-45b9-9309-c272f1438d7e"]
pub struct Terrain {
    size: Vec3,
    heightmap: Handle<Image>,
    mesh: Handle<Mesh>,
}

impl Terrain {
    pub fn new(size: Vec3, heightmap: Handle<Image>) -> Self {
        Terrain {
            size,
            heightmap,
            mesh: Default::default(),
        }
    }
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

type QueryCondition = Or<(Changed<Handle<Terrain>>, Added<Handle<Terrain>>)>;

fn terrain_mesh_linker(
    mut commands: Commands,
    mut terrain_events: EventReader<AssetEvent<Terrain>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut terrains: ResMut<Assets<Terrain>>,
    images: Res<Assets<Image>>,
    mut query: Query<(
        Entity,
        &Handle<Terrain>,
        &mut Handle<Mesh>,
    )>,
    changed_handles: Query<Entity, QueryCondition>,
) {
    for event in terrain_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                for (.., mut mesh) in query.iter_mut()
                    .filter(|(_, terrain, ..)| terrain == &handle)
                {
                    let terrain = terrains.get_mut(handle).unwrap();
                    mesh::generate_mesh(terrain, &mut meshes, &images);
            
                    info!(
                        "Terrain '{:?}' created. Adding mesh component to entity.",
                        terrain
                    );

                    *mesh = terrain.mesh.clone().clone();
                }
            }
            AssetEvent::Modified { handle } => {
                for (.., mut mesh) in query.iter_mut()
                    .filter(|(_, terrain, ..)| terrain == &handle)
                {
                    let terrain = terrains.get(handle).unwrap();
                    mesh::regenerate_mesh(terrain, &mut meshes, &images);

                    info!(
                        "Terrain '{:?}' modified. Changing mesh component of entity.",
                        terrain
                    );

                    if mesh.deref() != &terrain.mesh.clone() {
                        let old_mesh = mesh.clone();
                        *mesh = terrain.mesh.clone().clone();
                        meshes.remove(old_mesh);
                    }
                }
            }
            AssetEvent::Removed { handle } => {
                for (entity, ..) in query.iter_mut().filter(|(_, terrain, ..)| terrain == &handle) {
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }

    for entity in changed_handles.iter() {
        let Ok((.., handle, mut mesh))
            = query.get_mut(entity) else { continue };
        let Some(terrain) = terrains.get(handle) else { continue };
        
        info!(
            "Terrain handle for entity '{:?}' modified. Changing mesh component of entity.",
            entity
        );

        *mesh = terrain.mesh.clone().clone();
    }
}