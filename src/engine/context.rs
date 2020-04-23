use wgpu::{
    Adapter, CommandEncoderDescriptor, Device, Queue, RenderPipeline, Surface, SwapChain,
    SwapChainDescriptor,
};
use winit::window::Window;

use super::frame::Frame;

pub struct GfxContext {
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface,
    pub sc_desc: SwapChainDescriptor,
    pub size: (u32, u32),
    pub swapchain: SwapChain,
    pub window: Window,
}
impl GfxContext {
    pub async fn new(window: Window) -> Self {
        let (win_width, win_height) = (window.inner_size().width, window.inner_size().height);
        let surface = Surface::create(&window);
        let adapter = wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            },
            wgpu::BackendBit::PRIMARY,
        )
        .await
        .expect("Failed to find a suitable adapter");
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                extensions: wgpu::Extensions {
                    anisotropic_filtering: false,
                },
                limits: Default::default(),
            })
            .await;
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: win_width,
            height: win_height,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        let swapchain = device.create_swap_chain(&surface, &sc_desc);

        Self {
            size: (win_width, win_height),
            swapchain,
            device,
            queue,
            sc_desc,
            adapter,
            surface,
            window,
        }
    }
    pub fn next_frame(&mut self) -> Frame {
        let frame = self
            .swapchain
            .get_next_texture()
            .expect("Timeout getting texture");

        let encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        Frame::new(encoder, frame)
    }
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = (new_size.width, new_size.height);
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swapchain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }
}
