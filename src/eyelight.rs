use renderer::{RendererBase, AbstractRenderer};
use rng::{Rng, MathRng};
use scene::Scene;
use std::rand::SeedableRng;
use math::{vec2, vec2s, vec3, vec3s};
use ray::Isect;

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
        let scene = self.base.scene;

        let res_x = scene.camera.resolution.x as u32;
        let res_y = scene.camera.resolution.y as u32;

        for pix_id in range(0, res_x * res_y) {
            let x = pix_id % res_x;
            let y = pix_id / res_x;

            let sample = vec2(x as f32, y as f32) +
                if iteration == 0 { vec2s(0.5) } else { self.rng.get_vec2f() };

            let ray = scene.camera.generate_ray(sample);
            let mut isect = Isect { dist: 1e36, ..Isect::new() };

            if scene.intersect(&ray, &mut isect) {
                let dot_ln = isect.normal.dot(-ray.dir);

                self.base.framebuffer.add_color(sample,
                    if dot_ln > 0.0 { vec3s(dot_ln) }
                    else { vec3(-dot_ln, 0.0, 0.0) });
            }
        }

        self.base.iterations += 1;
    }
}
