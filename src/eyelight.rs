use renderer::{RendererBase, AbstractRenderer};
use rng::Rng;
use scene::Scene;
use std::rand::SeedableRng;

pub struct EyeLight<'a> {
    base: RendererBase<'a>,
    rng: Rng
}

impl<'a> EyeLight<'a> {
    pub fn new(scene: &Scene, seed: u32) -> EyeLight {
        EyeLight {
            base: RendererBase::new(scene),
            rng: SeedableRng::from_seed([0, seed as u64]),
        }
    }
}

impl<'a> AbstractRenderer<'a> for EyeLight<'a> {
    fn base<'b>(&'b self) -> &'b RendererBase<'a> {
        &self.base
    }

    fn base_mut<'b>(&'b mut self) -> &'b mut RendererBase<'a> {
        &mut self.base
    }

    fn run_iteration(&mut self, iteration: u32) {
        unimplemented!(); // TODO
    }
}
