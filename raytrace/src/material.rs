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

pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.make_unit_vector();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some((uv - n * dt) * ni_over_nt - n * discriminant.sqrt())
    } else {
        None
    }
}

pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    pub ref_idx: f32,
}

impl Dielectric {
    pub fn scatter(&self, ray: Ray, hit: HitRecord, rng: &mut ThreadRng) -> Option<Scatter> {
        let reflected = ray.direction.reflect(hit.normal);
        let attenuation = Vec3::ones();
        let (outward_normal, ni_over_nt, cosine) = if ray.direction.dot(hit.normal) > 0.0 {
            (-hit.normal, self.ref_idx,
             self.ref_idx * ray.direction.dot(hit.normal) / ray.direction.length())
        } else {
            (hit.normal, 1.0 / self.ref_idx,
             -ray.direction.dot(hit.normal) / ray.direction.length())
        };
        if let Some(refracted) = refract(ray.direction, outward_normal, ni_over_nt) {
            let reflection_prob = schlick(cosine, self.ref_idx);
            if rng.gen::<f32>() < reflection_prob {
                Some(Scatter::new(attenuation, Ray::new(hit.p, reflected)))
            } else {
                Some(Scatter::new(attenuation, Ray::new(hit.p, refracted)))
            }
        } else {
            Some(Scatter::new(attenuation, Ray::new(hit.p, reflected)))
        }
    }
}

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material {
    pub fn lambertian(albedo: Vec3) -> Material {
        Material::Lambertian(Lambertian { albedo })
    }

    pub fn metal(albedo: Vec3, fuzz: f32) -> Material {
        Material::Metal(Metal { albedo, fuzz })
    }

    pub fn dielectric(ref_idx: f32) -> Material {
        Material::Dielectric(Dielectric { ref_idx })
    }

    pub fn scatter(&self, ray: Ray, hit: HitRecord, rng: &mut ThreadRng) -> Option<Scatter> {
        match hit.material {
            Material::Lambertian(l) => l.scatter(ray, hit, rng),
            Material::Metal(m) => m.scatter(ray, hit, rng),
            Material::Dielectric(d) => d.scatter(ray, hit, rng),
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
