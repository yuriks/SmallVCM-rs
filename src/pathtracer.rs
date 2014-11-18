use renderer::{RendererBase, AbstractRenderer};
use rng::{Rng, MathRng};
use scene::Scene;
use std::rand::SeedableRng;
use math::{vec2, vec2s, vec3, vec3s};
use ray::Isect;

pub struct PathTracer<'a> {
    base: RendererBase<'a>,
    rng: Rng,
}

impl<'a> PathTracer<'a> {
    pub fn new(scene: &Scene, seed: u32) -> PathTracer {
        PathTracer {
            base: RendererBase::new(scene),
            rng: SeedableRng::from_seed([0, seed as u64]),
        }
    }
}

impl<'a> AbstractRenderer<'a> for PathTracer<'a> {
    fn base<'b>(&'b self) -> &'b RendererBase<'a> {
        &self.base
    }

    fn base_mut<'b>(&'b mut self) -> &'b mut RendererBase<'a> {
        &mut self.base
    }

    fn run_iteration(&mut self, iteration: u32) {
        let scene = self.base.scene;

        let light_count = scene.get_light_count();
        let light_pick_prob = 1.0 / light_count as f32;

        let res_x = scene.camera.resolution.x as u32;
        let res_y = scene.camera.resolution.y as u32;

        for pix_id in range(0, res_x * res_y) {
            let x = pix_id % res_x;
            let y = pix_id / res_x;

            let sample = vec2(x as f32, y as f32) + self.rng.get_vec2f();

            let ray = scene.camera.generate_ray(sample);
            let mut isect = Isect { dist: 1e36, ..Isect::new() };

            let path_weight = vec3s(1.0);
            let color = vec3s(0.0);
            let mut path_length = 1;
            let last_specular = true;
            let last_pdf_w = 1.0f32;

            loop {
                if !scene.intersect(&ray, &mut isect) {
                    if path_length < self.base.min_path_length {
                        break;
                    }

                    let background = scene.get_background();
                    if background.is_none() {
                        break;
                    }
                }

                path_length += 1;
            }
        }
    }
}
