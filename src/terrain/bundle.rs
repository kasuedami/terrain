use bevy::prelude::Bundle;

use bevy::prelude::*;

use super::{Terrain, material::TerrainMaterial};

#[derive(Bundle, Default)]
pub struct TerrainBundle {
    pub terrain: Handle<Terrain>,
    pub mesh: Handle<Mesh>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub material: Handle<TerrainMaterial>,
}