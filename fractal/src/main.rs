extern crate rand;
extern crate num;
extern crate palette;
extern crate quicksilver;
#[macro_use]
extern crate stdweb;

use std::str;
use rand::Rng;
use num::complex::Complex;
use palette::{Hsv, Srgb, FromColor};
use quicksilver::{
    Future, Result,
    combinators::ok,
    load_file,
    geom::{Rectangle, Vector, Shape},
    graphics::{Background::Col, Color, PixelFormat, Image},
    input::{ButtonState, Key},
    lifecycle::{Asset, Event, Settings, State, Window, run},
    prelude::Img,
};

mod text;
use crate::{
    text::{Text, TextRenderer},
};

const WIN_X: u32 = 800;
const WIN_Y: u32 = 800;

// mandelbrot consts
const MAX_ITER: u8 = 80;
const RE_START: f32 = -2.;
const RE_END: f32 = 1.;
const IM_START: f32 = -1.5;
const IM_END: f32 = 1.5;

enum Fractal {
    None,
    SierpinskiTriangle { img: Image },
    Mandelbrot { img: Image },
}

struct FractalRenderer {
    run: bool,
    elapsed: f64,
    steps: usize,
    fractal: Fractal,
    text_renderer: TextRenderer,
}

impl State for FractalRenderer {
    fn new() -> Result<FractalRenderer> {
        let fractal = change_fractal(&Fractal::None);
        let text_renderer = TextRenderer::new();
        Ok(FractalRenderer {run: true, elapsed: 0., steps: 0, fractal: fractal, text_renderer: text_renderer})
    }

    fn event(&mut self, event: &Event, _window: &mut Window) -> Result<()> {
        if let Event::Key(Key::Space, ButtonState::Pressed) = event {
            self.run = !self.run;
        }
        if let Event::Key(Key::R, ButtonState::Pressed) = event {
            self.steps = 0;
            self.elapsed = 0.;
            self.fractal = match &self.fractal {
                Fractal::None => Fractal::None,
                Fractal::SierpinskiTriangle { img } => new_sierpinski_triangle(WIN_X, WIN_Y),
                Fractal::Mandelbrot { img } => new_mandelbrot(WIN_X, WIN_Y),
            }
        }
        if let Event::Key(Key::C, ButtonState::Pressed) = event {
            self.steps = 0;
            self.elapsed = 0.;
            self.fractal = change_fractal(&self.fractal);
        }
        Ok(())
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if self.run {
            self.elapsed += window.update_rate();
            // run at 5fps
            while self.elapsed / 200. > self.steps as f64 {
                //TODO: animation???
                self.steps += 1;
            }
        }
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        match &self.fractal {
            Fractal::None => (),
            Fractal::SierpinskiTriangle { img } => {
                window.draw(&img.area().with_center((WIN_X/2, WIN_Y/2)), Img(&img));
            }
            Fractal::Mandelbrot { img } => {
                window.draw(&img.area().with_center((WIN_X/2, WIN_Y/2)), Img(&img));
            }
        }

        self.text_renderer.draw(
            window,
            (WIN_X as f32 - 75., 20.),
            &Text::Number(self.steps as i32),
        )?;
        self.text_renderer.draw(
            window,
            (140., 20.),
            &Text::Space,
        )?;
        self.text_renderer.draw(
            window,
            (90., 40.),
            &Text::R,
        )?;
        self.text_renderer.draw(
            window,
            (135., 60.),
            &Text::C,
        )?;
        
        Ok(())
    }
}

fn main() {
    run::<FractalRenderer>("FractalRenderer", Vector::new(WIN_X, WIN_Y), Settings::default());
}

fn change_fractal(current: &Fractal) -> Fractal {
    match current {
        Fractal::None => new_mandelbrot(WIN_X, WIN_Y),
        Fractal::Mandelbrot { img } => new_sierpinski_triangle(WIN_X, WIN_Y),
        Fractal::SierpinskiTriangle { img } => new_mandelbrot(WIN_X, WIN_Y),
    }
}

fn new_sierpinski_triangle(max_x: u32, max_y: u32) -> Fractal {
    // allocate pixel buffer (RGBA - 4 * u8 per pixel)
    let mut pixels = vec![0 as u8; (4 * max_x * max_y) as usize];
    // use bitwise arithmetic fill pixels where: x & y == 0
    let mut rng = rand::thread_rng();
    let red: u8 = rng.gen();
    let green: u8 = rng.gen();
    let blue: u8 = rng.gen();
    for y in 0..max_y {
        for x in 0..max_x {
            if x & y == 0 {
                // draw pixel
                let index = 4 * (x + y * max_x) as usize;
                let bytes = [red, green, blue, 255];
                for i in 0..bytes.len() {
                    pixels[index + i] = bytes[i];
                }
            }
        }
    }
    match Image::from_raw(pixels.as_slice(), max_y as u32, max_y as u32, PixelFormat::RGBA) {
        Ok(img) => {
            Fractal::SierpinskiTriangle { img: img }
        }
        Err(msg) => {
            //debug_output(msg),
            Fractal::None
        }
    }
}

fn new_mandelbrot(max_x: u32, max_y: u32) -> Fractal {
    let mut rng = rand::thread_rng();
    let max_iter = rng.gen_range(10, MAX_ITER);
    // allocate pixel buffer (RGBA - 4 * u8 per pixel)
    let mut pixels = vec![0 as u8; (4 * max_x * max_y) as usize];
    // color the pixels
    for y in 0..max_y {
        for x in 0..max_x {
            // convert pixel coord to complex number
            let c = Complex::new(
                RE_START + (x as f32 / max_x as f32) * (RE_END - RE_START),
                IM_START + (y as f32 / max_y as f32) * (IM_END - IM_START));
            // compute iterations
            let m = calc_mandelbrot_point(c, max_iter);
            // color depends on the num of iterations
            let hue = m / max_iter as f32 * 360.;
            let saturation = 1.;
            let value = if m < max_iter as f32 { 1. } else { 0. };
            let hsv = Hsv::new(hue, saturation, value);
            let rgb = Srgb::from_hsv(hsv);
            // draw pixel
            let index = 4 * (x + y * max_x) as usize;
            let bytes = [
                (rgb.red * 255.) as u8,
                (rgb.green * 255.) as u8,
                (rgb.blue * 255.) as u8,
                255];
            for i in 0..bytes.len() {
                pixels[index + i] = bytes[i];
            }
        }
    }
    match Image::from_raw(pixels.as_slice(), max_y as u32, max_y as u32, PixelFormat::RGBA) {
        Ok(img) => {
            Fractal::Mandelbrot { img: img }
        }
        Err(msg) => {
            //debug_output(msg),
            Fractal::None
        }
    }
}

fn complex_abs(c: Complex<f32>) -> f32 {
    (c.re * c.re + c.im * c.im).sqrt()
}

fn calc_mandelbrot_point(c: Complex<f32>, max_iter: u8) -> f32 {
    let mut z = Complex::new(0., 0.);
    let mut n = 0;
    while complex_abs(z) <= 2. && n < max_iter {
        z = z*z + c;
        n += 1;
    }
    // dont just return n, 'renormalize the mandelbrot escape'
    if n == max_iter {
        max_iter as f32
    }
    else {
        n as f32 + 1. - complex_abs(z).log2().ln()
    }
}

#[cfg(target_arch = "wasm32")]
fn debug_output(msg: &str) {
    js! {
        console.log(@{msg});
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn debug_output(msg: &str) {
    println!("{}", msg);
}
