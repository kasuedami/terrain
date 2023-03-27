use bevy::prelude::Bundle;

use bevy::prelude::*;

use super::Terrain;

#[derive(Bundle)]
pub struct TerrainBundle {
    pub terrain: Handle<Terrain>,
    pub mesh: Handle<Mesh>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

impl Default for TerrainBundle {
    fn default() -> Self {
        Self {
            terrain: Default::default(),
            mesh: Default::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
        }
    }
}