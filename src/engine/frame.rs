use wgpu::{Color, CommandEncoder, SwapChainOutput};

use super::context::GfxContext;

pub struct Frame {
    pub encoder: CommandEncoder,
    pub frame: SwapChainOutput,
}

impl Frame {
    pub fn new(encoder: CommandEncoder, frame: SwapChainOutput) -> Self {
        Self { encoder, frame }
    }
    pub fn finish(self, ctx: &GfxContext) {
        ctx.queue.submit(&[self.encoder.finish()]);
    }
}
