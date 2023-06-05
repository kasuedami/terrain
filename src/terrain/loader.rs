use bevy::{asset::{AssetLoader, LoadedAsset}, prelude::Vec3};

use serde::{Deserialize, Serialize};

use crate::terrain::material::TerrainMaterial;

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

    // let heightmap_handle = context.get_handle(asset.heightmap_path);
    let atlas_handle = context.get_handle(asset.atlas_path);
    let layer_red_texture_handle = context.get_handle(asset.layer_red_texture_path);
    let layer_green_texture_handle = context.get_handle(asset.layer_green_texture_path);
    let layer_blue_texture_handle = context.get_handle(asset.layer_blue_texture_path);

    let terrain = TerrainMaterial::new(
        atlas_handle,
        Some(layer_red_texture_handle),
        None,
        Some(layer_green_texture_handle),
        None,
        Some(layer_blue_texture_handle),
        None
    );

    // LoadedAsset::new(terrain)
    //     .with_dependencies(vec![
    //         asset.atlas_path.into(),
    //         asset.layer_red_texture_path.into(),
    //         asset.layer_green_texture_path.into(),
    //         asset.layer_blue_texture_path.into()
    //     ]);

    context.set_default_asset(LoadedAsset::new(terrain));
    
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