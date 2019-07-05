use rand::prelude::*;

use crate::{
    vec3::{Vec3},
    ray::{Ray},
};

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f32,
}

impl Camera {
    // vfov is top to bottom in degrees
    pub fn new(look_from: Vec3, look_to: Vec3, vup: Vec3, vfov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Self {
        let lens_radius = aperture / 2.0;
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let origin = look_from;
        let w = (look_from - look_to).make_unit_vector();
        let u = vup.cross(w).make_unit_vector();
        let v = w.cross(u);
        let lower_left_corner = origin - u * half_width * focus_dist - v * half_height * focus_dist - w * focus_dist;
        let horizontal = u * half_width * focus_dist * 2.0;
        let vertical = v * half_height * focus_dist * 2.0;
        Camera {
            origin: origin,
            lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical,
            u: u,
            v: v,
            w: w,
            lens_radius: lens_radius,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32, rng: &mut ThreadRng) -> Ray {
        let rd = random_in_unit_disk(rng) * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(self.origin + offset,
                 self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset)
    }
}

fn random_in_unit_disk(rng: &mut ThreadRng) -> Vec3 {
    loop {
        let p = Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), 0.0) * 2.0 - Vec3::new(1.0, 1.0, 0.0);
        if p.dot(p) < 1.0 {
            return p;
        }
    }
}
