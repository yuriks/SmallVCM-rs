use math::{Vec3f, vec3, vec3s};
use std::num::Float;

pub struct Frame {
    pub x: Vec3f,
    pub y: Vec3f,
    pub z: Vec3f,
}

impl Frame {
    pub fn identity() -> Frame {
        Frame {
            x: vec3(1.0, 0.0, 0.0),
            y: vec3(0.0, 1.0, 0.0),
            z: vec3(0.0, 0.0, 1.0),
        }
    }

    pub fn new(x: Vec3f, y: Vec3f, z: Vec3f) -> Frame {
        Frame {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn from_z(z: Vec3f) -> Frame {
        let tmp_z = z.normalized();
        let tmp_x = if tmp_z.x.abs() > 0.99 { vec3(0.0, 1.0, 0.0) } else { vec3(1.0, 0.0, 0.0) };
        let tmp_y = tmp_z.cross(tmp_x).normalized();

        Frame {
            x: tmp_y.cross(tmp_z),
            y: tmp_y,
            z: tmp_z,
        }
    }

    pub fn to_world(&self, a: Vec3f) -> Vec3f {
        self.x * vec3s(a.x) + self.y * vec3s(a.y) + self.z * vec3s(a.z)
    }

    pub fn to_local(&self, a: Vec3f) -> Vec3f {
        vec3(a.dot(self.x), a.dot(self.y), a.dot(self.z))
    }

    pub fn binormal(&self) -> Vec3f { self.x }
    pub fn tangent (&self) -> Vec3f { self.y }
    pub fn normal  (&self) -> Vec3f { self.z }
}
