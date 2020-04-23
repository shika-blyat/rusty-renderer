mod engine;

use futures::executor;
use wgpu::{
    Color, PrimitiveTopology, RenderPassColorAttachmentDescriptor, RenderPassDescriptor,
    RenderPipeline,
};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use engine::{
    context::GfxContext, frame::Frame, pipeline::PipelineDescriptor, shader::ShaderDescriptor,
};

fn render_triangle(frame: &mut Frame, pipeline: &RenderPipeline) {
    let mut render_pass = frame.encoder.begin_render_pass(&RenderPassDescriptor {
        color_attachments: &[RenderPassColorAttachmentDescriptor {
            attachment: &frame.frame.view,
            resolve_target: None,
            load_op: wgpu::LoadOp::Clear,
            store_op: wgpu::StoreOp::Store,
            clear_color: Color {
                r: 0.2,
                g: 0.5,
                b: 0.2,
                a: 1.0,
            },
        }],
        depth_stencil_attachment: None,
    });
    render_pass.set_pipeline(pipeline);
    render_pass.draw(0..3, 0..1);
}
fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(500.0, 500.0))
        .build(&event_loop)
        .expect("Failed to create window");
    let mut ctx = executor::block_on(GfxContext::new(window));
    let shader_desc = ShaderDescriptor {
        vertex_shader: "shaders/hardcode_shader.vert",
        frag_shader: "shaders/hardcode_shader.frag",
    };
    let pipeline_desc = PipelineDescriptor {
        alpha_blending: true,
        primitive_topo: PrimitiveTopology::TriangleList,
    };
    let pipeline = pipeline_desc.build(&ctx, shader_desc);
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    ctx.resize(*new_inner_size);
                }
                WindowEvent::Resized(physical_size) => {
                    ctx.resize(physical_size);
                }
                WindowEvent::CloseRequested => {
                    println!("The close button was pressed. stopping");
                    *control_flow = ControlFlow::Exit
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                let mut frame = ctx.next_frame();
                render_triangle(&mut frame, &pipeline);
                frame.finish(&ctx);
            }
            _ => (),
        }
    })
}
