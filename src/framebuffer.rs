use math::{Vec2f, Vec3f, vec2, vec3s};
use utils::luminance;
use std::path::Path;
use std::io::{File, IoResult};
use std::num::{Float, FloatMath};

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

    pub fn save_ppm(&self, _filename: &Path, _gamma: f32) {
        unimplemented!() // TODO
    }

    pub fn save_pfm(&self, _filename: &Path) {
        unimplemented!() // TODO
    }

    pub fn save_bmp(&self, filename: &Path, gamma: f32) -> IoResult<()> {
        const HEADER_SIZE: uint = 52;

        let mut bmp = try!(File::create(filename));
        try!(bmp.write(b"BM"));
        try!(bmp.write_le_u32((HEADER_SIZE + 2 + self.res_x * self.res_y * 3) as u32));
        try!(bmp.write_le_u32(0));
        try!(bmp.write_le_u32(HEADER_SIZE as u32 + 2));
        try!(bmp.write_le_u32(40));
        try!(bmp.write_le_i32(self.res_x as i32));
        try!(bmp.write_le_i32(self.res_y as i32));
        try!(bmp.write_le_u16(1));
        try!(bmp.write_le_u16(24));
        try!(bmp.write_le_u32(0));
        try!(bmp.write_le_u32((self.res_x * self.res_y * 3) as u32));
        try!(bmp.write_le_u32(2953));
        try!(bmp.write_le_u32(2953));
        try!(bmp.write_le_u32(0));
        try!(bmp.write_le_u32(0));

        let inv_gamma = 1.0 / gamma;
        for y in range(0, self.res_y) {
            for x in range(0, self.res_x) {
                let rgbf = self.color[x + (self.res_y - y - 1) * self.res_x];
                let gamma_bgr: [f32, ..3] = [
                    rgbf.z.powf(inv_gamma) * 255.0,
                    rgbf.y.powf(inv_gamma) * 255.0,
                    rgbf.x.powf(inv_gamma) * 255.0,
                ];

                let bgrb: [u8, ..3] = [
                    gamma_bgr[0].min(255.0).max(0.0) as u8,
                    gamma_bgr[1].min(255.0).max(0.0) as u8,
                    gamma_bgr[2].min(255.0).max(0.0) as u8,
                ];

                try!(bmp.write(bgrb[]));
            }
        }

        Ok(())
    }

    pub fn save_hdr(&self, _filename: &Path) -> IoResult<()> {
        unimplemented!() // TODO
    }
}
