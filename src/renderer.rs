use framebuffer::Framebuffer;
use scene::Scene;

pub trait AbstractRenderer<'a> {
    fn base<'b>(&'b self) -> &'b RendererBase<'a>;
    fn base_mut<'b>(&'b mut self) -> &'b mut RendererBase<'a>;
    fn run_iteration(&mut self, iteration: u32);
}

pub struct RendererBase<'a> {
    pub max_path_length: u32,
    pub min_path_length: u32,

    // originally protected
    pub iterations: u32,
    pub framebuffer: Framebuffer,
    pub scene: &'a Scene,
}

impl<'a> RendererBase<'a> {
    pub fn new(scene: &Scene) -> RendererBase {
        let mut framebuffer = Framebuffer::new();
        framebuffer.setup(scene.camera.resolution);

        RendererBase {
            max_path_length: 0,
            min_path_length: 2,
            iterations: 0,
            framebuffer: framebuffer,
            scene: scene,
        }
    }

    pub fn get_framebuffer(&self) -> Framebuffer {
        let mut framebuffer = self.framebuffer.clone();

        if self.iterations > 0 {
            framebuffer.scale(1.0 / self.iterations as f32);
        }

        framebuffer
    }

    pub fn was_used(&self) -> bool {
        self.iterations > 0
    }
}
