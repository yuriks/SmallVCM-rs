use framebuffer::Framebuffer;
use scene::Scene;

pub trait AbstractRenderer {
    fn get_base<'a>(&'a mut self) -> &mut RendererBase<'a>;
    fn run_iteration(&mut self, iteration: u32);
}

pub struct RendererBase<'a> {
    pub max_path_length: u32,
    pub min_path_length: u32,

    // originally protected
    pub iterations: u32,
    pub framebuffer: &'a Framebuffer,
    pub scene: &'a Scene,
}

impl<'a> RendererBase<'a> {
    fn get_framebuffer(&mut self) -> Framebuffer {
        let mut framebuffer = (*self.framebuffer).clone();

        if self.iterations > 0 {
            framebuffer.scale(1.0 / self.iterations as f32);
        }

        framebuffer
    }

    fn was_used(&self) -> bool {
        self.iterations > 0
    }
}
