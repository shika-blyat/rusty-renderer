use wgpu::{Color, CommandEncoder, SwapChainOutput};

use super::context::GfxContext;

pub struct Frame<'a> {
    pub encoder: CommandEncoder,
    pub frame: SwapChainOutput,
    pub ctx: &'a GfxContext,
}

impl<'a> Frame<'a> {
    pub fn new(encoder: CommandEncoder, frame: SwapChainOutput, ctx: &'a GfxContext) -> Self {
        Self {
            encoder,
            frame,
            ctx,
        }
    }
    pub fn finish(self) {
        self.ctx.queue.submit(&[self.encoder.finish()]);
    }
}
