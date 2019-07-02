use std::f32;

extern crate minifb;
use minifb::{Key, WindowOptions, Window};

mod vec3;
mod ray;
mod hitable;
use crate::{
    vec3::{Vec3},
    ray::{Ray},
    hitable::{Sphere, World},
};

const WIDTH: usize = 640;
const HEIGHT: usize = 320;

fn color(ray: Ray, world: &World) -> Vec3 {
    if let Some(hit) = world.hit(ray, 0.0, f32::MAX) {
        return Vec3::new(hit.normal.x + 1.0, hit.normal.y + 1.0, hit.normal.z + 1.0) * 0.5;
    } else {
        let unit_direction = ray.direction.make_unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t)  + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Test - ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::zeros();
    let world = World::new(vec![
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0),
    ]);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut c = 0;
        for i in buffer.iter_mut() {
            // calc x and y pixel
            let px = c % WIDTH;
            let py = HEIGHT - c / WIDTH;
            // make ray
            let u = px as f32 / WIDTH as f32;
            let v = py as f32 / HEIGHT as f32;
            let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            // calc color
            let col = color(ray, &world);
            // convert to u8
            let r = (255.0 * col.x) as u32;
            let g = (255.0 * col.y) as u32;
            let b = (255.0 * col.z) as u32;
            // set pixel
            *i = 0;
            *i |= r << 16;
            *i |= g << 8;
            *i |= b;
            // increment counter
            c += 1;
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer).unwrap();

        Vec3::new(0.0, 1.0, 2.0);
    }
}
