use wgpu::{Device, Extent3d, SwapChainDescriptor, TextureDimension};

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: Option<wgpu::Sampler>,
}

impl Texture {
    const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn create_depth_texture(sc_desc: &SwapChainDescriptor, device: &Device) -> Self {
        let desc = wgpu::TextureDescriptor {
            label: Some("Depth texture"),
            size: Extent3d {
                width: sc_desc.width,
                height: sc_desc.height,
                depth: 1,
            },
            mip_level_count: 1,
            array_layer_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: sc_desc.usage,
        };
        let texture = device.create_texture(&desc);
        let view = texture.create_default_view();
        Self {
            texture,
            view,
            sampler: None,
        }
    }
}
