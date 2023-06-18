use bevy::{asset::{AssetLoader, LoadedAsset}, prelude::Vec3};

use serde::{Deserialize, Serialize};

use crate::terrain::{Terrain, Layer};

#[derive(Default)]
pub struct TerrainLoader;

impl AssetLoader for TerrainLoader {
    fn extensions(&self) -> &[&str] {
        &["terrain"]
    }

    fn load<'a>(
            &'a self,
            bytes: &'a [u8],
            load_context: &'a mut bevy::asset::LoadContext,
        ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move { load_terrain(bytes, load_context).await })
    }

}

async fn load_terrain<'de, 'a, 'b>(
    bytes: &'a [u8],
    context: &'a mut bevy::asset::LoadContext<'b>,
) -> Result<(), bevy::asset::Error> {
    let asset: TerrainAsset = ron::de::from_bytes(bytes)?;

    let heightmap_handle = context.get_handle(asset.heightmap_path.clone());
    let atlas_handle = context.get_handle(asset.atlas_path.clone());
    let layer_red_texture_handle = context.get_handle(asset.layer_red_texture_path.clone());
    let layer_green_texture_handle = context.get_handle(asset.layer_green_texture_path.clone());
    let layer_blue_texture_handle = context.get_handle(asset.layer_blue_texture_path.clone());

    let mesh = crate::terrain::mesh::generate(asset.size, heightmap_handle.clone());
    let mesh_handle = context.set_labeled_asset("mesh", LoadedAsset::new(mesh));

    let terrain = Terrain::new(
        asset.size,
        heightmap_handle,
        atlas_handle,
        Some(layer_red_texture_handle),
        Layer::default(),
        Some(layer_green_texture_handle),
        Layer::default(),
        Some(layer_blue_texture_handle),
        Layer::default(),
        mesh_handle,
    );

    context.set_default_asset(
        LoadedAsset::new(terrain)
            .with_dependencies(vec![
                asset.heightmap_path.into(),
                asset.atlas_path.into(),
                asset.layer_red_texture_path.into(),
                asset.layer_green_texture_path.into(),
                asset.layer_blue_texture_path.into(),
            ])
        );
    
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TerrainAsset {
    size: Vec3,
    heightmap_path: String,
    atlas_path: String,
    layer_red_texture_path: String,
    layer_green_texture_path: String,
    layer_blue_texture_path: String,
}

#[test]
fn ron() {
    let terrain = TerrainAsset {
        size: Vec3 { x: 10.0, y: 2.0, z: 10.0 },
        heightmap_path: "images/Heightmap.png".into(),
        atlas_path: "images/atlas.png".into(),
        layer_red_texture_path: "images/grass.jpeg".into(),
        layer_green_texture_path: "images/stone_dark.jpeg".into(),
        layer_blue_texture_path: "images/stone_light.jpeg".into(),
    };

    println!(
        "{:?}",
        ron::ser::to_string_pretty(&terrain, ron::ser::PrettyConfig::default())
    );
}