use math::{Vec2f, Vec3f, vec2, vec3s};
use utils::luminance;

#[deriving(Clone)]
pub struct Framebuffer {
    color: Vec<Vec3f>,
    resolution: Vec2f,
    res_x: uint,
    res_y: uint,
}

impl Framebuffer {
    pub fn new() -> Framebuffer {
        Framebuffer {
            color: Vec::new(),
            resolution: vec2(0.0, 0.0),
            res_x: 0,
            res_y: 0,
        }
    }

    pub fn add_color(&mut self, sample: Vec2f, color: Vec3f) {
        if sample.x < 0.0 || sample.x >= self.resolution.x {
            return;
        }
        if sample.y < 0.0 || sample.y >= self.resolution.y {
            return;
        }

        let (x, y) = (sample.x as uint, sample.y as uint);
        //let src = self.color[x + y * self.res_x];
        self.color[mut][x + y * self.res_x] = self.color[x + y * self.res_x] + color;
    }

    pub fn setup(&mut self, resolution: Vec2f) {
        let res_x = resolution.x as uint;
        let res_y = resolution.y as uint;

        self.color.clear();
        self.color.grow(res_x * res_y, vec3s(0.0));
        self.resolution = resolution;
        self.res_x = res_x;
        self.res_y = res_y;
    }

    pub fn clear(&mut self) {
        for x in self.color.iter_mut() {
            *x = vec3s(0.0);
        }
    }

    pub fn add(&mut self, other: &Framebuffer) {
        for (dest, src) in self.color.iter_mut().zip(other.color.iter()) {
            *dest = *dest + *src;
        }
    }

    pub fn scale(&mut self, scale: f32) {
        for dest in self.color.iter_mut() {
            *dest = *dest * vec3s(scale);
        }
    }

    pub fn total_luminance(&self) -> f32 {
        self.color.iter().fold(0.0, |a, &b| a + luminance(b))
    }

    pub fn save_ppm(&self, _filename: &str, _gamma: f32) {
        unimplemented!() // TODO
    }

    pub fn save_pfm(&self, _filename: &str) {
        unimplemented!() // TODO
    }

    pub fn save_bmp(&self, _filename: &str, _gamma: f32) {
        unimplemented!() // TODO
    }

    pub fn save_hdr(&self, _filename: &str) {
        unimplemented!() // TODO
    }
}
