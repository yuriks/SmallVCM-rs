use ray::{Ray, Isect};
use math::{Vec3f, vec3s};
use std::num::{Float, FloatMath};

pub trait AbstractGeometry {
    fn intersect(&self, ray: &Ray, result: &mut Isect) -> bool;

    fn intersect_p(&self, ray: &Ray, result: &mut Isect) -> bool {
        self.intersect(ray, result)
    }

    fn grow_bbox(&self, bbox_min: &mut Vec3f, bbox_max: &mut Vec3f);
}

pub struct GeometryList {
    pub geometry: Vec<Box<AbstractGeometry + 'static>>
}

impl GeometryList {
    pub fn new() -> GeometryList {
        GeometryList {
            geometry: Vec::new()
        }
    }
}

impl AbstractGeometry for GeometryList {
    fn intersect(&self, ray: &Ray, result: &mut Isect) -> bool {
        let mut any_intersection = false;
        for geometry in self.geometry.iter() {
            any_intersection |= geometry.intersect(ray, result);
        }
        any_intersection
    }

    fn intersect_p(&self, ray: &Ray, result: &mut Isect) -> bool {
        for geometry in self.geometry.iter() {
            if geometry.intersect_p(ray, result) {
                return true;
            }
        }
        false
    }

    fn grow_bbox(&self, bbox_min: &mut Vec3f, bbox_max: &mut Vec3f) {
        for geometry in self.geometry.iter() {
            geometry.grow_bbox(bbox_min, bbox_max);
        }
    }
}

pub struct Triangle {
    p: [Vec3f, ..3],
    mat_id: int,
    normal: Vec3f,
}

impl Triangle {
    pub fn new(p0: Vec3f, p1: Vec3f, p2: Vec3f, mat_id: int) -> Triangle {
        Triangle {
            p: [p0, p1, p2],
            mat_id: mat_id,
            normal: (p1 - p0).cross(p2 - p0).normalized(),
        }
    }
}

impl AbstractGeometry for Triangle {
    fn intersect(&self, ray: &Ray, result: &mut Isect) -> bool {
        let ao = self.p[0] - ray.org;
        let bo = self.p[1] - ray.org;
        let co = self.p[2] - ray.org;

        let v0 = co.cross(bo);
        let v1 = bo.cross(ao);
        let v2 = ao.cross(co);

        let v0d = v0.dot(ray.dir);
        let v1d = v1.dot(ray.dir);
        let v2d = v2.dot(ray.dir);

        if (v0d < 0.0 && v1d < 0.0 && v2d < 0.0) ||
           (v0d >= 0.0 && v1d >= 0.0 && v2d >= 0.0) {
            let distance = self.normal.dot(ao) / self.normal.dot(ray.dir);

            if distance > ray.tmin && distance < result.dist {
                result.normal = self.normal;
                result.mat_id = self.mat_id;
                result.dist = distance;
                return true;
            }
        }

        false
    }

    fn grow_bbox(&self, bbox_min: &mut Vec3f, bbox_max: &mut Vec3f) {
        for i in range(0, 3) {
            for j in range(0, 3) {
                let min = (*bbox_min)[j].min(self.p[i][j]);
                bbox_min[j] = min;
                let max = (*bbox_max)[j].max(self.p[i][j]);
                bbox_max[j] = max;
            }
        }
    }
}

pub struct Sphere {
    pub center: Vec3f,
    pub radius: f32,
    pub mat_id: int,
}

impl Sphere {
    pub fn new(center: Vec3f, radius: f32, mat_id: int) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            mat_id: mat_id,
        }
    }
}

impl AbstractGeometry for Sphere {
    fn intersect(&self, ray: &Ray, result: &mut Isect) -> bool {
        let transformed_origin = ray.org - self.center;

        let a = ray.dir.dot(ray.dir);
        let b = 2.0 * ray.dir.dot(transformed_origin);
        let c = transformed_origin.dot(transformed_origin) - self.radius * self.radius;

        let disc = (b*b - 4.0*a*c) as f64;

        if disc < 0.0 {
            return false;
        }

        let disc_sqrt = disc.sqrt();
        let q = if b < 0.0 {
            (-b as f64 - disc_sqrt) / 2.0
        } else {
            (-b as f64 + disc_sqrt) / 2.0
        };

        let t0 = (q / a as f64) as f32;
        let t1 = (c as f64 / q) as f32;

        let (t0, t1) = if t0 > t1 { (t1, t0) } else { (t0, t1) };

        let res_t = if t0 > ray.tmin && t0 < result.dist {
            t0
        } else if t1 > ray.tmin && t1 < result.dist {
            t1
        } else {
            return false;
        };

        result.dist = res_t as f32;
        result.mat_id = self.mat_id;
        result.normal = (transformed_origin + vec3s(res_t) * ray.dir).normalized();
        return true;
    }

    fn grow_bbox(&self, bbox_min: &mut Vec3f, bbox_max: &mut Vec3f) {
        for i in range(0u, 8) {
            let mut p = self.center;
            let mut half = vec3s(self.radius);

            for j in range(0, 3) {
                if (i & (1 << j)) != 0 {
                    half[j] = -half[j];
                }
            }

            p = p + half;

            for j in range(0, 3) {
                bbox_min[j] = (*bbox_min)[j].min(p[j]);
                bbox_max[j] = (*bbox_max)[j].max(p[j]);
            }
        }
    }
}
