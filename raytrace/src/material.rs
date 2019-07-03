use rand::prelude::*;

use crate::{
    vec3::{Vec3},
    ray::{Ray},
    hitable::{HitRecord},
};

pub struct Scatter {
    pub attenuation: Vec3,
    pub ray: Ray,
}

impl Scatter {
    pub fn new(attenuation: Vec3, ray: Ray) -> Scatter {
        Scatter { attenuation, ray }
    }
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn scatter(&self, _ray: Ray, hit: HitRecord, rng: &mut ThreadRng) -> Option<Scatter> {
        let target = hit.p + hit.normal + random_in_unit_sphere(rng);
        let scattered = Ray::new(hit.p, target - hit.p);
        let attenuation = self.albedo;
        Some(Scatter::new(attenuation, scattered))
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn scatter(&self, ray: Ray, hit: HitRecord, rng: &mut ThreadRng) -> Option<Scatter> {
        let reflected = ray.direction.make_unit_vector().reflect(hit.normal);
        let scattered = Ray::new(hit.p, reflected + random_in_unit_sphere(rng) * self.fuzz);
        let attenuation = self.albedo;
        if scattered.direction.dot(hit.normal) > 0.0 {
            return Some(Scatter::new(attenuation, scattered));
        }
        None
    }
}

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Material {
    pub fn lambertian(albedo: Vec3) -> Material {
        Material::Lambertian(Lambertian { albedo })
    }

    pub fn metal(albedo: Vec3, fuzz: f32) -> Material {
        Material::Metal(Metal { albedo, fuzz })
    }

    pub fn scatter(&self, ray: Ray, hit: HitRecord, rng: &mut ThreadRng) -> Option<Scatter> {
        match hit.material {
            Material::Lambertian(l) => l.scatter(ray, hit, rng),
            Material::Metal(m) => m.scatter(ray, hit, rng),
        }
    }
}

fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
    loop {
        let p = Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) * 2.0 - Vec3::ones();
        if p.squared_length() < 1.0 {
            break p;
        }
    }
}
