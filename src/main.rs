mod engine;

use futures::executor;
use wgpu::{Color, PrimitiveTopology};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use engine::{
    clear_screen::ClearScreen, context::GfxContext, mesh::Mesh, pipeline::PipelineDescriptor,
    shader::ShaderDescriptor, vertex::Vertex,
};

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.5, -0.5, 0.8],
        color: [0.5, 0.0, 0.5, 1.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.8],
        color: [0.5, 0.0, 0.5, 1.0],
    },
    Vertex {
        position: [0.5, 0.5, 0.8],
        color: [0.5, 0.0, 0.5, 1.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.5, 0.8, 0.5, 1.0],
    },
    Vertex {
        position: [0.5, 0.5, 0.0],
        color: [0.5, 0.8, 0.5, 1.0],
    },
    Vertex {
        position: [-0.5, 0.5, 0.0],
        color: [0.5, 0.8, 0.5, 1.0],
    },
];

const INDICES: &[u32] = &[0, 1, 2, 3, 4, 5];

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(500.0, 500.0))
        .build(&event_loop)
        .expect("Failed to create window");
    let mut ctx = executor::block_on(GfxContext::new(window));
    let shader_desc = ShaderDescriptor {
        vertex_shader: "shaders/shader.vert",
        frag_shader: "shaders/shader.frag",
    };
    let pipeline_desc = PipelineDescriptor {
        alpha_blending: true,
        primitive_topo: PrimitiveTopology::TriangleList,
    };
    let pipeline = pipeline_desc.build(&ctx, shader_desc);
    let clear_screen = ClearScreen::new(
        &ctx,
        Color {
            r: 0.5,
            g: 0.5,
            b: 0.5,
            a: 1.0,
        },
    );
    let mesh = Mesh::new(&ctx, VERTICES, INDICES);
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
                clear_screen.draw(&mut frame);
                mesh.draw(&mut frame, &pipeline);
                frame.finish();
            }
            _ => (),
        }
    })
}
