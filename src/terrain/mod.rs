use std::ops::Deref;

use bevy::{prelude::*, reflect::TypeUuid};

use self::material::TerrainMaterial;

pub mod bundle;
mod mesh;
pub mod material;

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
            
                    info!("Some terrain created. Adding mesh component to entity.");

                    *mesh = terrain.mesh.clone().clone();
                }
            }
            AssetEvent::Modified { handle } => {
                for (.., mut mesh) in query.iter_mut()
                    .filter(|(_, terrain, ..)| terrain == &handle)
                {
                    let terrain = terrains.get(handle).unwrap();
                    mesh::regenerate_mesh(terrain, &mut meshes, &images);

                    info!("Some terrain modified. Changing mesh component of entity.");

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