use wgpu::{
    Buffer, Color, RenderPassColorAttachmentDescriptor, RenderPassDescriptor, RenderPipeline,
};

use super::{context::GfxContext, frame::Frame, vertex::Vertex};

pub struct Mesh {
    vertex: Buffer,
    indices_len: u32,
    indices: Buffer,
}

impl Mesh {
    pub fn new<'a>(ctx: &GfxContext, vertex: &'a [Vertex], indices: &'a [u32]) -> Self {
        let indices_len = indices.len() as u32;
        let vertex = ctx
            .device
            .create_buffer_with_data(bytemuck::cast_slice(vertex), wgpu::BufferUsage::VERTEX);
        let indices = ctx
            .device
            .create_buffer_with_data(bytemuck::cast_slice(indices), wgpu::BufferUsage::INDEX);
        Self {
            vertex,
            indices_len,
            indices,
        }
    }
    pub fn draw(&self, frame: &mut Frame, pipeline: &RenderPipeline) {
        let mut render_pass = frame.encoder.begin_render_pass(&RenderPassDescriptor {
            color_attachments: &[RenderPassColorAttachmentDescriptor {
                attachment: &frame.frame.view,
                resolve_target: None,
                load_op: wgpu::LoadOp::Load,
                store_op: wgpu::StoreOp::Store,
                clear_color: Color::BLUE, // Ignored anyway
            }],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                attachment: &frame.ctx.depth_texture.view,
                depth_load_op: wgpu::LoadOp::Clear,
                depth_store_op: wgpu::StoreOp::Store,
                clear_depth: 1.0,
                stencil_load_op: wgpu::LoadOp::Clear,
                stencil_store_op: wgpu::StoreOp::Store,
                clear_stencil: 0,
            }),
        });
        render_pass.set_pipeline(pipeline);
        render_pass.set_vertex_buffer(0, &self.vertex, 0, 0);
        render_pass.set_index_buffer(&self.indices, 0, 0);
        render_pass.draw_indexed(0..self.indices_len, 0, 0..1);
    }
}
