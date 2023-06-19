use bevy::{asset::{AssetLoader, io::Reader, LoadContext, AsyncReadExt}, prelude::{Vec3, Image}};

use serde::{Deserialize, Serialize};

use crate::terrain::{Terrain, Layer};

#[derive(Default)]
pub(crate) struct TerrainLoader;

impl AssetLoader for TerrainLoader {
    type Asset = Terrain;
    type Settings = ();

    fn extensions(&self) -> &[&str] {
        &["terrain"]
    }

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        load_context: &'a mut LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, anyhow::Error>> {
        Box::pin(async move { load_terrain(reader, load_context).await })
    }
}

async fn load_terrain<'a, 'b>(
    reader: &'a mut Reader<'_>,
    context: &'a mut bevy::asset::LoadContext<'b>,
) -> Result<<TerrainLoader as AssetLoader>::Asset, anyhow::Error> {
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes).await?;

    let asset: TerrainAsset = ron::de::from_bytes(&bytes)?;
    let heightmap: Image = context.load_direct(&asset.heightmap_path).await?.take::<Image>().unwrap();

    let heightmap_handle = context.load(&asset.heightmap_path);
    let atlas_handle = context.load(&asset.atlas_path);
    let layer_red_texture_handle = context.load(&asset.layer_red_texture_path);
    let layer_green_texture_handle = context.load(&asset.layer_green_texture_path);
    let layer_blue_texture_handle = context.load(&asset.layer_blue_texture_path);


    let mesh = crate::terrain::mesh::generate(asset.size, heightmap);
    let mesh_handle = context.add_labeled_asset("mesh".to_owned(), mesh);

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

    Ok(terrain)
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