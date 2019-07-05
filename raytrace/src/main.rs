use std::f32;

extern crate minifb;
use minifb::{Key, WindowOptions, Window};
use rand::prelude::*;

mod vec3;
mod ray;
mod hitable;
mod camera;
mod material;
mod renderer;
use crate::{
    vec3::{Vec3},
    hitable::{Sphere, World},
    camera::{Camera},
    material::{Material},
};

const WIDTH: usize = 800;
const HEIGHT: usize = 400;

fn random_scene(rng: &mut ThreadRng) -> Vec<Sphere> {
    let n = 5;
    let mut spheres = Vec::new();
    spheres.push(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Material::lambertian(Vec3::new(0.5, 0.5, 0.5))));
    for a in -n..n {
        for b in -n..n {
            let choose_mat = rng.gen::<f32>();
            let center = Vec3::new(a as f32 + 0.9 * rng.gen::<f32>(), 0.2, b as f32 + 0.9 * rng.gen::<f32>());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 { // diffuse
                    spheres.push(Sphere::new(center, 0.2,
                                 Material::lambertian(Vec3::new(rng.gen::<f32>() * rng.gen::<f32>(),
                                                                rng.gen::<f32>() * rng.gen::<f32>(),
                                                                rng.gen::<f32>() * rng.gen::<f32>()))));
                } else if choose_mat < 0.95 { // metal
                    spheres.push(Sphere::new(center, 0.2,
                                 Material::metal(Vec3::new(0.5 * (1.0 + rng.gen::<f32>()),
                                                           0.5 * (1.0 + rng.gen::<f32>()),
                                                           0.5 * (1.0 + rng.gen::<f32>())),
                                                 0.5 * rng.gen::<f32>())));
                } else { // glass
                    spheres.push(Sphere::new(center, 0.2, Material::dielectric(1.5)));
                }
            }
        }
    }
    spheres.push(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Material::dielectric(1.5)));
    spheres.push(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Material::lambertian(Vec3::new(0.4, 0.2, 0.1))));
    spheres.push(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Material::metal(Vec3::new(0.7, 0.6, 0.5), 0.0)));
    spheres
}

fn main() {
    let mut window = Window::new("Test - ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let look_from = Vec3::new(15.0, 2.0, 4.0);
    let look_to = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = (look_from - look_to).length();
    let aperture = 0.05;
    let camera = Camera::new(look_from, look_to, Vec3::new(0.0, 1.0, 0.0), 15.0, WIDTH as f32 / HEIGHT as f32, aperture, dist_to_focus);
    let mut rng = rand::thread_rng();
    let world = World::new(random_scene(&mut rng));

    let buffer = renderer::render(WIDTH, HEIGHT, camera, world);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer).unwrap();
    }
}
