extern crate minifb;
use minifb::{Key, WindowOptions, Window};

mod vec3;
mod ray;
use crate::{
    vec3::{Vec3},
    ray::{Ray},
};

const WIDTH: usize = 640;
const HEIGHT: usize = 320;

fn hit_sphere(center: Vec3, radius: f32, r: Ray) -> bool {
    let oc = r.origin - center;
    let a = r.direction.dot(r.direction);
    let b = 2.0 * oc.dot(r.direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - a * c * 4.0;
    discriminant > 0.0
}

fn color(r: Ray) -> Vec3 {
    if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r) {
        Vec3::new(1.0, 0.0, 0.0)
    }
    else {
        let unit_direction = r.direction.make_unit_vector();
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
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut c = 0;
        for i in buffer.iter_mut() {
            // calc x and y pixel
            let px = c % WIDTH;
            let py = c / WIDTH;
            // make ray
            let u = px as f32 / WIDTH as f32;
            let v = py as f32 / HEIGHT as f32;
            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            // calc color
            let col = color(r);
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
