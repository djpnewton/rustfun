use std::f32;

extern crate minifb;
use minifb::{Key, WindowOptions, Window};
use rand::prelude::*;

mod vec3;
mod ray;
mod hitable;
mod camera;
use crate::{
    vec3::{Vec3},
    ray::{Ray},
    hitable::{Sphere, World},
    camera::{Camera},
};

const WIDTH: usize = 640;
const HEIGHT: usize = 320;
const SAMPLES: usize = 100;

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

    let camera = Camera::new();
    let world = World::new(vec![
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0),
    ]);
    let mut rng = rand::thread_rng();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut c = 0;
        for pixel in buffer.iter_mut() {
            // calc x and y pixel
            let px = c % WIDTH;
            let py = HEIGHT - c / WIDTH;
            // make color
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _n in 1..=SAMPLES {
                // calc color for this sample
                let u = (px as f32 + rng.gen::<f32>()) / WIDTH as f32;
                let v = (py as f32 + rng.gen::<f32>()) / HEIGHT as f32;
                let ray = camera.get_ray(u, v);
                let col_temp = color(ray, &world); 
                col = col + col_temp; // add all color values together
            }
            col = col / SAMPLES as f32; // get average of color values
            // convert to u8
            let r = (255.99 * col.x) as u32;
            let g = (255.99 * col.y) as u32;
            let b = (255.99 * col.z) as u32;
            // set pixel
            *pixel = 0;
            *pixel |= r << 16;
            *pixel |= g << 8;
            *pixel |= b;
            // increment counter
            c += 1;
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer).unwrap();

        println!("ok");
    }
}
