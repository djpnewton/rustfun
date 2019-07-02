extern crate minifb;
use minifb::{Key, WindowOptions, Window};

mod vec3;
use crate::{
    vec3::{Vec3},
};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;


fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Test - ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut c = 0;
        for i in buffer.iter_mut() {
            // clear pixel
            *i = 0;
            // calc rgb components
            let col = Vec3::new((c % WIDTH) as f32 / WIDTH as f32, (c / WIDTH) as f32 / HEIGHT as f32, 0.2);
            // convert to u8
            let ri = (255.0 * col.r()) as u32;
            let gi = (255.0 * col.g()) as u32;
            let bi = (255.0 * col.b()) as u32;
            // set pixel
            *i |= ri << 16;
            *i |= gi << 8;
            *i |= bi;
            // increment counter
            c += 1;
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer).unwrap();

        Vec3::new(0.0, 1.0, 2.0);
    }
}
