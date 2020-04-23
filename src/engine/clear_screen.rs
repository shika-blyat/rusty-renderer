use super::{context::GfxContext, frame::Frame, shader::compile_shader};

use wgpu::{Color, RenderPipeline};

pub struct ClearScreen {
    clear_color: Color,
    pipeline: RenderPipeline,
}

impl ClearScreen {
    pub fn new(ctx: &GfxContext, clear_color: Color) -> Self {
        let layout = ctx
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[],
            });
        let module = compile_shader(
            ctx,
            "shaders/empty_shader.vert",
            glsl_to_spirv::ShaderType::Vertex,
        );
        let pipeline_desc = &wgpu::RenderPipelineDescriptor {
            layout: &layout,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &module,
                entry_point: "main",
            },
            fragment_stage: None,
            rasterization_state: None,
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: ctx.sc_desc.format,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: None,
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint32,
                vertex_buffers: &[],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        };
        Self {
            pipeline: ctx.device.create_render_pipeline(&pipeline_desc),
            clear_color,
        }
    }

    pub fn draw(&self, frame: &mut Frame) {
        let mut render_pass = frame
            .encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.frame.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: self.clear_color,
                }],
                depth_stencil_attachment: None,
            });
        render_pass.set_pipeline(&self.pipeline);
        render_pass.draw(0..0, 0..0);
    }
}
