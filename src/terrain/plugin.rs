use bevy::prelude::*;

use super::{Terrain, loader};

#[derive(Default)]
pub struct TerrainPlugin {
    editor_mode: bool,
}

impl TerrainPlugin {
    pub fn editor_mode() -> TerrainPlugin {
        TerrainPlugin { editor_mode: true }
    }
}

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_asset::<Terrain>()
            .add_plugin(MaterialPlugin::<Terrain>::default())
            .add_systems(Update, terrain_mesh_linker)
            .init_asset_loader::<loader::TerrainLoader>();

        if self.editor_mode {
            app.add_plugin(super::editor::EditorPlugin);
        }
    }
}

type QueryCondition = Or<(Changed<Handle<Terrain>>, Added<Handle<Terrain>>)>;

fn terrain_mesh_linker(
    mut commands: Commands,
    mut terrain_events: EventReader<AssetEvent<Terrain>>,
    mut terrains: ResMut<Assets<Terrain>>,
    mut query: Query<(
        Entity,
        &Handle<Terrain>,
        &mut Handle<Mesh>,
    )>,
    changed_handles: Query<Entity, QueryCondition>,
) {
    for event in terrain_events.iter() {
        match event {
            AssetEvent::Added { id, .. } => {
                for (.., mut mesh) in query.iter_mut()
                    .filter(|(_, terrain, ..)| terrain.id() == *id)
                {
                    let terrain = terrains.get_mut(*id).unwrap();                
                 
                    *mesh = terrain.mesh.clone().clone();
                }
            },
            AssetEvent::Removed { id } => {
                for (entity, ..) in query.iter_mut().filter(|(_, terrain, ..)| terrain.id() == *id) {
                    commands.entity(entity).despawn_recursive();
                }
            },
            _ => ()
        }
    }

    for entity in changed_handles.iter() {
        let Ok((.., handle, mut mesh))
            = query.get_mut(entity) else { continue };
        let Some(terrain) = terrains.get(handle) else { continue };

        *mesh = terrain.mesh.clone().clone();
    }
}