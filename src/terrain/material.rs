use std::num::NonZeroU32;

use bevy::{
    prelude::*,
    render::{render_resource::{
        AsBindGroup,
        ShaderRef, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, ShaderStages, SamplerBindingType, TextureSampleType, TextureViewDimension, AsBindGroupError, PreparedBindGroup, BindGroupDescriptor, BindGroupEntry, BindingResource
    }, renderer::RenderDevice, texture::FallbackImage, render_asset::RenderAssets},
    reflect::TypeUuid
};

#[derive(Debug, Clone, TypeUuid)]
#[uuid = "cb732d71-3adc-4ebe-b7c1-3e92a7186f29"]
pub struct TerrainMaterial {
    atlas: Handle<Image>,
    layers: [Option<TerrainLayer>; 4],
}

impl TerrainMaterial {
    pub fn new(atlas: Handle<Image>, layers: &[TerrainLayer]) -> Self {
        let mut list: [Option<TerrainLayer>; 4] = Default::default();
        for i in 0..4 {
            if i < layers.len() {
                list[i] = Some(layers[i].clone());
            } else {
                list[i] = None;
            }
        }

        TerrainMaterial { atlas, layers: list }
    }
}

impl AsBindGroup for TerrainMaterial {
    type Data = ();

    fn as_bind_group(
        &self,
        layout: &BindGroupLayout,
        render_device: &RenderDevice,
        image_assets: &RenderAssets<Image>,
        fallback_image: &FallbackImage,
    ) -> Result<PreparedBindGroup<Self::Data>, AsBindGroupError> {

        let atlas = match image_assets.get(&self.atlas) {
            Some(image) => &*image.texture_view,
            None => return Err(AsBindGroupError::RetryNextUpdate),
        };

        let mut images = vec![];
        for layer in &self.layers {
            match layer {
                Some(layer) => {
                    match image_assets.get(&layer.texture) {
                        Some(image) => images.push(&*image.texture_view),
                        None => return Err(AsBindGroupError::RetryNextUpdate),
                    }
                },
                None => {
                    images.push(&*fallback_image.texture_view);
                }
            }
        }
        
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            label: "terrain_bind_group".into(),
            layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Sampler(&fallback_image.sampler),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&atlas),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::TextureViewArray(&images[..]),
                },
            ],
        });

        Ok(PreparedBindGroup {
            bindings: vec![],
            bind_group,
            data: (),
        })
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout
    where
        Self: Sized {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: "terrain_material_layout".into(),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: NonZeroU32::new(4 as u32),
                }
            ]
        })
    }
}

impl Material for TerrainMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/terrain_shader.wgsl".into()
    }
}

#[derive(Debug, Clone)]
pub struct TerrainLayer {
    texture: Handle<Image>,
    scaling: Vec2,
}

impl TerrainLayer {
    pub fn new(texture: Handle<Image>, scaling: Vec2) -> Self {
        TerrainLayer { texture, scaling }
    }
}