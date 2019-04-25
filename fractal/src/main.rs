extern crate rand;
extern crate quicksilver;
#[macro_use]
extern crate stdweb;

use std::str;
use rand::Rng;
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

enum Fractal {
    SierpinskiTriangle { color: Color },
}

struct Map {
    x: usize,
    y: usize,
    state: Vec<bool>,
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
    let color = Color::from_rgba(rng.gen(), rng.gen(), rng.gen(), 1.);
    Fractal::SierpinskiTriangle { color: color }
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
