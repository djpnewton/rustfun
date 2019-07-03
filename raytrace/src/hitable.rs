use crate::{
    vec3::{Vec3},
    ray::{Ray},
    material::{Material},
};

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Sphere {
        Sphere { center, radius, material }
    }

    pub fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp  < t_max && temp > t_min {
                let hit_point = ray.point_at_parameter(temp);
                return Some(HitRecord {
                    t: temp,
                    p: hit_point,
                    normal: (hit_point - self.center) / self.radius,
                    material: self.material,
                });
            } 
        } 
        None
    }
}

pub struct World {
    pub spheres: Vec<Sphere>,
}

impl World {
    pub fn new(spheres: Vec<Sphere>) -> World {
        World { spheres }
    }

    pub fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_result: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for sphere in self.spheres.iter() {
            if let Some(hit) = sphere.hit(ray, t_min, closest_so_far) {
                closest_so_far = if hit.t < closest_so_far {
                    hit_result = Some(hit);
                    hit.t
                } else {
                    closest_so_far
                }
            }
        }
        hit_result
    }
}
