use math::{Vec3f, Vec2f, Mat4f, vec3, PI};

pub struct Camera {
    position: Vec3f,
    forward: Vec3f,
    resolution: Vec2f,
    raster_to_world: Mat4f,
    world_to_raster: Mat4f,
    image_plane_dist: f32,
}

impl Camera {
    pub fn new(position: Vec3f, forward: Vec3f, up: Vec3f,
               resolution: Vec2f, horizontal_fov: f32) -> Camera {
        let forward = forward.normalized();
        let up = up.cross(-forward).normalized();
        let left = -forward.cross(up);

        let pos = vec3(up.dot(position),
                       left.dot(position),
                      -forward.dot(position));

        let mut world_to_camera = Mat4f::identity();
        world_to_camera.set_row_vec3(0, up,       -pos.x);
        world_to_camera.set_row_vec3(1, left,     -pos.y);
        world_to_camera.set_row_vec3(2, -forward, -pos.z);

        let perspective = Mat4f::perspective(horizontal_fov, 0.1, 10000.0);
        let world_to_nscreen = perspective * world_to_camera;
        let nscreen_to_world = world_to_nscreen.inverted();

        let tan_half_angle = (horizontal_fov * PI / 360.0).tan();

        Camera {
            position: position,
            forward: forward,
            resolution: resolution,
            raster_to_world: nscreen_to_world *
                             Mat4f::translate(&vec3(-1.0, -1.0, 0.0)) *
                             Mat4f::scale(&vec3(2.0 / resolution.x, 2.0 / resolution.y, 0.0)),
            world_to_raster: Mat4f::scale(&vec3(resolution.x * 0.5, resolution.y * 0.5, 0.0)) *
                             Mat4f::translate(&vec3(1.0, 1.0, 0.0)) *
                             world_to_nscreen,
            image_plane_dist: resolution.x / (2.0 * tan_half_angle),
        }
    }
}
