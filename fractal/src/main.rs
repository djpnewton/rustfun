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
    geom::{Rectangle, Vector},
    graphics::{Background::Col, Color},
    input::{ButtonState, Key},
    lifecycle::{Asset, Event, Settings, State, Window, run},
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
const IM_START: f32 = -1.;
const IM_END: f32 = 1.;

enum Fractal {
    SierpinskiTriangle { color: Color },
    Mandelbrot,
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
        let fractal = choose_fractal();
        let text_renderer = TextRenderer::new();
        Ok(FractalRenderer {run: false, elapsed: 0., steps: 0, fractal: fractal, text_renderer: text_renderer})
    }

    fn event(&mut self, event: &Event, _window: &mut Window) -> Result<()> {
        if let Event::Key(Key::Space, ButtonState::Pressed) = event {
            self.run = !self.run;
        }
        if let Event::Key(Key::R, ButtonState::Pressed) = event {
            self.steps = 0;
            self.elapsed = 0.;
            self.fractal = choose_fractal();
        }
        if let Event::Key(Key::C, ButtonState::Pressed) = event {
            self.steps = 0;
            self.elapsed = 0.;
            self.fractal = choose_fractal();
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

        match self.fractal {
            Fractal::SierpinskiTriangle { color } => {
                // use bitwise arithmetic fill pixels where: x & y == 0
                for y in 0..WIN_Y {
                    for x in 0..WIN_X {
                        if x & y == 0 {
                            window.draw(&Rectangle::new((x, y), (1, 1)), Col(color));
                        }
                    }
                }
            }
            Fractal::Mandelbrot => {
                for y in 0..WIN_Y {
                    for x in 0..WIN_X {
                        // convert pixel coord to complex number
                        let c = Complex::new(
                            RE_START + (x as f32 / WIN_X as f32) * (RE_END - RE_START),
                            IM_START + (y as f32 / WIN_Y as f32) * (IM_END - IM_START));
                        // compute iterations
                        let m = calc_mandelbrot(c);
                        // color depends on the num of iterations
                        let hue = m / MAX_ITER as f32 * 360.;
                        let saturation = 1.;
                        let value = if m < MAX_ITER as f32 { 1. } else { 0. };
                        let hsv = Hsv::new(hue, saturation, value);
                        let rgb = Srgb::from_hsv(hsv);
                        // draw pixel
                        let color = Color::from_rgba(
                            (rgb.red * 255.) as u8, 
                            (rgb.green * 255.) as u8, 
                            (rgb.blue * 255.) as u8, 
                            1.);
                        window.draw(&Rectangle::new((x, y), (1, 1)), Col(color));
                    }
                }
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

fn choose_fractal() -> Fractal {
    let mut rng = rand::thread_rng();
    let n: u8 = rng.gen();
    if n % 2 == 0 {
        let color = Color::from_rgba(rng.gen(), rng.gen(), rng.gen(), 1.);
        Fractal::SierpinskiTriangle { color: color }
    } else {
        Fractal::Mandelbrot
    }
}

fn complex_abs(c: Complex<f32>) -> f32 {
    (c.re * c.re + c.im * c.im).sqrt()
}

fn calc_mandelbrot(c: Complex<f32>) -> f32 {
    let mut z = Complex::new(0., 0.);
    let mut n = 0;
    while complex_abs(z) <= 2. && n < MAX_ITER {
        z = z*z + c;
        n += 1;
    }
    // dont just return n, 'renormalize the mandelbrot escape'
    if n == MAX_ITER {
        MAX_ITER as f32
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
