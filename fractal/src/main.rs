extern crate rand;
extern crate num;
extern crate palette;
extern crate quicksilver;
#[macro_use]
extern crate stdweb;

use std::str;
use std::time::SystemTime;
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

const WIN_X: u32 = 400;
const WIN_Y: u32 = 400;
const PIXEL_AMOUNT_PER_PASS: u32 = WIN_X * WIN_Y / 10;

// mandelbrot consts
const MAX_ITER: u8 = 80;
const RE_START: f32 = -2.;
const RE_END: f32 = 1.;
const IM_START: f32 = -1.5;
const IM_END: f32 = 1.5;

struct FractalView {
    re_start: f32,
    re_end: f32,
    im_start: f32,
    im_end: f32,
}

trait Fractal {
    fn create_image(&mut self, sz_x: u32, sz_y: u32);
    fn get_image(&self) -> &Option<Image>;
}

trait MandelbrotFamily {
    fn translate_view(&mut self, re_delta: f32, im_delta: f32);
    fn zoom_view(&mut self, delta: f32);
}

struct SierpinskiTriangle {
    img: Option<Image>,
}

impl Fractal for SierpinskiTriangle {
    fn create_image(&mut self, sz_x: u32, sz_y: u32) {
        // allocate pixel buffer (RGBA - 4 * u8 per pixel)
        let mut pixels = vec![0 as u8; (4 * sz_x * sz_y) as usize];
        // use bitwise arithmetic fill pixels where: x & y == 0
        let mut rng = rand::thread_rng();
        let red: u8 = rng.gen();
        let green: u8 = rng.gen();
        let blue: u8 = rng.gen();
        for y in 0..sz_y {
            for x in 0..sz_x {
                if x & y == 0 {
                    // draw pixel
                    let index = 4 * (x + y * sz_x) as usize;
                    let bytes = [red, green, blue, 255];
                    for i in 0..bytes.len() {
                        pixels[index + i] = bytes[i];
                    }
                }
            }
        }
        match Image::from_raw(pixels.as_slice(), sz_x, sz_y, PixelFormat::RGBA) {
            Ok(img) => {
                self.img = Some(img);
            }
            Err(_e) => {
                self.img = None;
                //debug_output(_e),
            }
        }
    }
    fn get_image(&self) -> &Option<Image> {
        &self.img
    }
}

impl SierpinskiTriangle {
    fn new(sz_x: u32, sz_y: u32) -> SierpinskiTriangle {
        let mut tri = SierpinskiTriangle { img: None };
        tri.create_image(sz_x, sz_y);
        tri
    }
}

struct Mandelbrot {
    pixels_index: u32,
    pixels: Option<Vec<u8>>,
    width: u32,
    height: u32,
    img: Option<Image>,
    view: FractalView,
    max_iter: u8,
}

impl Fractal for Mandelbrot {
    fn create_image(&mut self, sz_x: u32, sz_y: u32) {
        // allocate pixel buffer (RGBA - 4 * u8 per pixel)
        self.pixels_index = 0;
        self.pixels = Some(vec![0 as u8; (4 * sz_x * sz_y) as usize]);
        self.width = sz_x;
        self.height = sz_y;
    }
    fn get_image(&self) -> &Option<Image> {
        &self.img
    }
}

impl Mandelbrot {
    fn new(view: FractalView, sz_x: u32, sz_y: u32) -> Mandelbrot {
        let mut rng = rand::thread_rng();
        let max_iter: u8 = rng.gen_range(10, MAX_ITER);
        let mut man = Mandelbrot { pixels_index: 0, pixels: None, width: 0, height: 0, view: view, img: None, max_iter: max_iter };
        man.create_image(sz_x, sz_y);
        man
    }
    fn reset_image(&mut self) {
        self.pixels_index = 0;
    }
    fn update_image(&mut self) {
        let last = self.pixels_index;
        let mut count = 0;
        // color the pixels
        match &mut self.pixels {
            Some(pixels) => {
                for y in 0..self.height {
                    for x in 0..self.width {
                        // only do a few pixels at a time
                        let current = y * self.width + x;
                        if current <= last {
                            continue;
                        }
                        if count > PIXEL_AMOUNT_PER_PASS {
                            break;
                        }
                        count += 1;
                        self.pixels_index = current;
                        // convert pixel coord to complex number
                        let c = Complex::new(
                            self.view.re_start + (x as f32 / self.width as f32) * (self.view.re_end - self.view.re_start),
                            self.view.im_start + (y as f32 / self.height as f32) * (self.view.im_end - self.view.im_start));
                        // compute iterations
                        let m = calc_mandelbrot_point(c, self.max_iter);
                        // color depends on the num of iterations
                        let hue = m / self.max_iter as f32 * 360.;
                        let saturation = 1.;
                        let value = if m < self.max_iter as f32 { 1. } else { 0. };
                        let hsv = Hsv::new(hue, saturation, value);
                        let rgb = Srgb::from_hsv(hsv);
                        // draw pixel
                        let index = 4 * (x + y * self.width) as usize;
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
                match Image::from_raw(pixels.as_slice(), self.width, self.height, PixelFormat::RGBA) {
                    Ok(img) => {
                        self.img = Some(img);
                    }
                    Err(_e) => {
                        self.img = None;
                        //debug_output(_e),
                    }
                }
            }
            None => (),
        }
    }
}

impl MandelbrotFamily for Mandelbrot {
    fn translate_view(&mut self, re_delta: f32, im_delta: f32) {
        let re_diff = (self.view.re_end - self.view.re_start) * re_delta;
        self.view.re_start += re_diff;
        self.view.re_end += re_diff;
        let im_diff = (self.view.im_end - self.view.im_start) * im_delta;
        self.view.im_start += im_diff;
        self.view.im_end += im_diff;
    }
    fn zoom_view(&mut self, delta: f32) {
        let re_diff = (self.view.re_end - self.view.re_start) * delta;
        self.view.re_start -= re_diff;
        self.view.re_end += re_diff;
        let im_diff = (self.view.im_end - self.view.im_start) * delta;
        self.view.im_start -= im_diff;
        self.view.im_end += im_diff;
    }
}

struct Julia {
    pixels_index: u32,
    pixels: Option<Vec<u8>>,
    width: u32,
    height: u32,
    img: Option<Image>,
    view: FractalView,
    max_iter: u8,
}

impl Fractal for Julia {
    fn create_image(&mut self, sz_x: u32, sz_y: u32) {
        // allocate pixel buffer (RGBA - 4 * u8 per pixel)
        self.pixels_index = 0;
        self.pixels = Some(vec![0 as u8; (4 * sz_x * sz_y) as usize]);
        self.width = sz_x;
        self.height = sz_y;
    }
    fn get_image(&self) -> &Option<Image> {
        &self.img
    }
}

impl Julia {
    fn new(view: FractalView, sz_x: u32, sz_y: u32) -> Julia {
        let mut rng = rand::thread_rng();
        let max_iter: u8 = rng.gen_range(10, MAX_ITER);
        let mut jul = Julia { pixels_index: 0, pixels: None, width: 0, height: 0, view: view, img: None, max_iter: max_iter };
        jul.create_image(sz_x, sz_y);
        jul
    }
    fn reset_image(&mut self) {
        self.pixels_index = 0;
    }
    fn update_image(&mut self) {
        let last = self.pixels_index;
        let mut count = 0;
        // c constant for the julia set
        let c = Complex::new(0.285, 0.01);
        // color the pixels
        match &mut self.pixels {
            Some(pixels) => {
                for y in 0..self.height {
                    for x in 0..self.width {
                        // only do a few pixels at a time
                        let current = y * self.width + x;
                        if current <= last {
                            continue;
                        }
                        if count > PIXEL_AMOUNT_PER_PASS {
                            break;
                        }
                        count += 1;
                        self.pixels_index = current;
                        // convert pixel coord to complex number
                        let z0 = Complex::new(
                            self.view.re_start + (x as f32 / self.width as f32) * (self.view.re_end - self.view.re_start),
                            self.view.im_start + (y as f32 / self.height as f32) * (self.view.im_end - self.view.im_start));
                        // compute iterations
                        let m = calc_julia_point(c, z0, self.max_iter);
                        // color depends on the num of iterations
                        let hue = m / self.max_iter as f32 * 360.;
                        let saturation = 1.;
                        let value = if m < self.max_iter as f32 { 1. } else { 0. };
                        let hsv = Hsv::new(hue, saturation, value);
                        let rgb = Srgb::from_hsv(hsv);
                        // draw pixel
                        let index = 4 * (x + y * self.width) as usize;
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
                match Image::from_raw(pixels.as_slice(), self.width, self.height, PixelFormat::RGBA) {
                    Ok(img) => {
                        self.img = Some(img);
                    }
                    Err(_e) => {
                        self.img = None;
                        //debug_output(_e),
                    }
                }
            }
            None => (),
        }
    }
}


impl MandelbrotFamily for Julia {
    fn translate_view(&mut self, re_delta: f32, im_delta: f32) {
        let re_diff = (self.view.re_end - self.view.re_start) * re_delta;
        self.view.re_start += re_diff;
        self.view.re_end += re_diff;
        let im_diff = (self.view.im_end - self.view.im_start) * im_delta;
        self.view.im_start += im_diff;
        self.view.im_end += im_diff;
    }
    fn zoom_view(&mut self, delta: f32) {
        let re_diff = (self.view.re_end - self.view.re_start) * delta;
        self.view.re_start -= re_diff;
        self.view.re_end += re_diff;
        let im_diff = (self.view.im_end - self.view.im_start) * delta;
        self.view.im_start -= im_diff;
        self.view.im_end += im_diff;
    }
}

enum Fractals {
    None,
    SierpinskiTriangle(SierpinskiTriangle), 
    Mandelbrot(Mandelbrot),
    Julia(Julia),
}

struct FractalRenderer {
    run: bool,
    elapsed: f64,
    steps: usize,
    fractal: Fractals,
    text_renderer: TextRenderer,
}

impl State for FractalRenderer {
    fn new() -> Result<FractalRenderer> {
        let fractal = change_fractal(&Fractals::None);
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
            let view = FractalView { re_start: RE_START, re_end: RE_END, im_start: IM_START, im_end: IM_END };
            self.fractal = match &self.fractal {
                Fractals::None => Fractals::None,
                Fractals::SierpinskiTriangle(_tri) => Fractals::SierpinskiTriangle(SierpinskiTriangle::new(WIN_X, WIN_Y)),
                Fractals::Mandelbrot(_man) => Fractals::Mandelbrot(Mandelbrot::new(view, WIN_X, WIN_Y)),
                Fractals::Julia(_jul) => Fractals::Julia(Julia::new(view, WIN_X, WIN_Y)),
            }
        }
        if let Event::Key(Key::C, ButtonState::Pressed) = event {
            self.steps = 0;
            self.elapsed = 0.;
            self.fractal = change_fractal(&self.fractal);
        }
        let mut translate_x = 0.;
        let mut translate_y = 0.;
        let mut delta_zoom = 0.;
        if let Event::Key(Key::Left, ButtonState::Pressed) = event {
            translate_x = 0.1;
        }
        if let Event::Key(Key::Right, ButtonState::Pressed) = event {
            translate_x = -0.1;
        }
        if let Event::Key(Key::Up, ButtonState::Pressed) = event {
            translate_y = 0.1;
        }
        if let Event::Key(Key::Down, ButtonState::Pressed) = event {
            translate_y = -0.1;
        }
        if let Event::Key(Key::Z, ButtonState::Pressed) = event {
            delta_zoom = 0.1;
        }
        if let Event::Key(Key::X, ButtonState::Pressed) = event {
            delta_zoom = -0.1;
        }
        if translate_x != 0. || translate_y != 0. {
            match &mut self.fractal {
                Fractals::None => (),
                Fractals::SierpinskiTriangle(_tri) => (),
                Fractals::Mandelbrot(man) => man.translate_view(translate_x, translate_y),
                Fractals::Julia(jul) => jul.translate_view(translate_x, translate_y),
            }
        }
        if delta_zoom != 0. {
            match &mut self.fractal {
                Fractals::None => (),
                Fractals::SierpinskiTriangle(_tri) => (),
                Fractals::Mandelbrot(man) => man.zoom_view(delta_zoom),
                Fractals::Julia(jul) => jul.zoom_view(delta_zoom),
            }
        }
        if translate_x != 0. || translate_y != 0. || delta_zoom != 0. {
            match &mut self.fractal {
                Fractals::None => (),
                Fractals::SierpinskiTriangle(_tri) => (),
                Fractals::Mandelbrot(man) => man.reset_image(),
                Fractals::Julia(jul) => jul.reset_image(),
            }
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
        match &mut self.fractal {
            Fractals::None => (),
            Fractals::SierpinskiTriangle(_tri) => (),
            Fractals::Mandelbrot(man) => man.update_image(),
            Fractals::Julia(jul) => jul.update_image(),
        }
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        let img = match &mut self.fractal {
            Fractals::None => &None,
            Fractals::SierpinskiTriangle(tri) => &tri.img,
            Fractals::Mandelbrot(man) => &man.img,
            Fractals::Julia(jul) => &jul.img,
        };
        match img {
            Some(img) => window.draw(&img.area().with_center((WIN_X/2, WIN_Y/2)), Img(&img)),
            None => (),
        };

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
            (110., 40.),
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

fn change_fractal(current: &Fractals) -> Fractals {
    let now = SystemTime::now();
    let view = FractalView { re_start: RE_START, re_end: RE_END, im_start: IM_START, im_end: IM_END };
    let fractal = match current {
        Fractals::None => Fractals::Mandelbrot(Mandelbrot::new(view, WIN_X, WIN_Y)),
        Fractals::Mandelbrot(_man) => Fractals::Julia(Julia::new(view, WIN_X, WIN_Y)),
        Fractals::Julia(_jul)  => Fractals::SierpinskiTriangle(SierpinskiTriangle::new(WIN_X, WIN_Y)),
        Fractals::SierpinskiTriangle(_tri) => Fractals::Mandelbrot(Mandelbrot::new(view, WIN_X, WIN_Y)),
    };
    match now.elapsed() {
        Ok(elapsed) => {
            debug_output(&format!("{}ms", elapsed.as_millis()));
        }
        Err(_e) => {}
    };
    fractal
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

fn calc_julia_point(c: Complex<f32>, z0: Complex<f32>, max_iter: u8) -> f32 {
    let mut z = z0;
    let mut n = 0;
    while complex_abs(z) <= 2. && n < max_iter {
        z = z*z + c;
        n += 1;
    }
    // dont just return n, 'renormalize the julia escape'
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
