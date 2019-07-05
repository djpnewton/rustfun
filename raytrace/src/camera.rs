use crate::{
    vec3::{Vec3},
    ray::{Ray},
};

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    // vfov is top to bottom in degrees
    pub fn new(look_from: Vec3, look_to: Vec3, vup: Vec3, vfov: f32, aspect: f32) -> Self {
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let origin = look_from;
        let w = (look_from - look_to).make_unit_vector();
        let u = vup.cross(w).make_unit_vector();
        let v = w.cross(u);
        let lower_left_corner = origin - u * half_width - v * half_height - w;
        let horizontal = u * half_width * 2.0;
        let vertical = v * half_height * 2.0;
        Camera {
            origin: origin,
            lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin)
    }
}
