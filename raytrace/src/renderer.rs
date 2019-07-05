use crate::camera::Camera;
use crate::hitable::World;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::prelude::*;
use rayon::prelude::*;

const NUM_SAMPLES: u32 = 128;
const MAX_DEPTH: u32 = 16;

fn color(ray: Ray, world: &World, depth: u32, rng: &mut ThreadRng) -> Vec3 {
    if let Some(hit) = world.hit(ray, 0.0001, std::f32::MAX) {
        if depth < MAX_DEPTH {
            if let Some(scatter) = hit.material.scatter(ray, hit, rng) {
                color(scatter.ray, world, depth + 1, rng) * scatter.attenuation
            } else {
                Vec3::zeros()
            }
        }
        else {
            Vec3::zeros()
        }
    } else {
        let unit_direction = ray.direction.make_unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        Vec3::ones() * (1.0 - t)  + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn to_bgra(r: u32, g: u32, b: u32) -> u32 {
    255 << 24 | r << 16 | g << 8 | b
}

pub fn render(width: usize, height: usize, camera: Camera, world: World) -> Vec<u32> {
    (0..width * height)
        .into_par_iter()
        .map_init(
            || thread_rng(),
            |mut rng, screen_pos| {
                let mut col = Vec3::zeros();
                let i = height - 1 - screen_pos / width;
                let j = screen_pos % width;
                for _ in 0..NUM_SAMPLES {
                    let u = ((j as f32) + rng.gen::<f32>()) / (width as f32);
                    let v = ((i as f32) + rng.gen::<f32>()) / (height as f32);
                    let ray = camera.get_ray(u, v, &mut rng);
                    col = col + color(ray, &world, 0, &mut rng);
                }
                col = col * (1.0 / NUM_SAMPLES as f32);
                let ir = (255.99 * col.x.sqrt()) as u32;
                let ig = (255.99 * col.y.sqrt()) as u32;
                let ib = (255.99 * col.z.sqrt()) as u32;

                to_bgra(ir, ig, ib)
            },
        )
        .collect()
}
