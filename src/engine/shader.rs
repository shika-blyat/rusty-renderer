use std::{fs::File, io::prelude::*, path::PathBuf};

use wgpu::ShaderModule;

use super::context::GfxContext;

pub struct ShaderDescriptor<'a> {
    pub vertex_shader: &'a str,
    pub frag_shader: &'a str,
}

pub fn compile_shader<'a>(
    ctx: &GfxContext,
    path: &'a str,
    kind: glsl_to_spirv::ShaderType,
) -> ShaderModule {
    let mut file =
        File::open(path).expect(format!("Failed to open {:#?} shader file", path).as_str());
    let mut src = String::new();
    file.read_to_string(&mut src)
        .expect("Failed to read the content of {}");
    let shader_spv = glsl_to_spirv::compile(&src, kind).expect("Failed to compile shader {:#}");
    let data = wgpu::read_spirv(shader_spv)
        .expect(format!("Failed to read spirv of {:#?}", path).as_str());
    let module = ctx.device.create_shader_module(&data);
    module
}
