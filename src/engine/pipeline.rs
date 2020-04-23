use super::{
    context::GfxContext,
    shader::{compile_shader, ShaderDescriptor},
    vertex::Vertex,
};
use wgpu::{
    BlendDescriptor, ColorWrite, CompareFunction, DepthStencilStateDescriptor, IndexFormat,
    PrimitiveTopology, RenderPipeline, StencilStateFaceDescriptor, TextureFormat,
    VertexStateDescriptor,
};

pub struct PipelineDescriptor {
    pub alpha_blending: bool,
    pub primitive_topo: PrimitiveTopology,
}

impl PipelineDescriptor {
    pub fn build(self, gfx: &GfxContext, shader_desc: ShaderDescriptor) -> RenderPipeline {
        let vs_module = compile_shader(
            &gfx,
            shader_desc.vertex_shader,
            glsl_to_spirv::ShaderType::Vertex,
        );
        let fs_module = compile_shader(
            &gfx,
            shader_desc.frag_shader,
            glsl_to_spirv::ShaderType::Fragment,
        );

        let render_pipeline_layout =
            gfx.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    bind_group_layouts: &[],
                });
        let color_states = if self.alpha_blending {
            [wgpu::ColorStateDescriptor {
                format: gfx.sc_desc.format,
                color_blend: wgpu::BlendDescriptor {
                    src_factor: wgpu::BlendFactor::SrcAlpha,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                    operation: wgpu::BlendOperation::Add,
                },
                alpha_blend: BlendDescriptor::REPLACE,
                write_mask: ColorWrite::ALL,
            }]
        } else {
            [wgpu::ColorStateDescriptor {
                format: gfx.sc_desc.format,
                color_blend: BlendDescriptor::REPLACE,
                alpha_blend: BlendDescriptor::REPLACE,
                write_mask: ColorWrite::ALL,
            }]
        };
        let render_pipeline_desc = wgpu::RenderPipelineDescriptor {
            layout: &render_pipeline_layout,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            primitive_topology: self.primitive_topo,
            color_states: &color_states,
            depth_stencil_state: Some(DepthStencilStateDescriptor {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil_front: StencilStateFaceDescriptor::IGNORE,
                stencil_back: StencilStateFaceDescriptor::IGNORE,
                stencil_read_mask: 0,
                stencil_write_mask: 0,
            }),
            vertex_state: VertexStateDescriptor {
                index_format: IndexFormat::Uint32,
                vertex_buffers: &[Vertex::desc()],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        };
        gfx.device.create_render_pipeline(&render_pipeline_desc)
    }
}
impl Default for PipelineDescriptor {
    fn default() -> Self {
        Self {
            alpha_blending: false,
            primitive_topo: PrimitiveTopology::PointList,
        }
    }
}
