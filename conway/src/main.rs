use std::fs;

extern crate quicksilver;

use quicksilver::{
    Result,
    geom::{Rectangle, Vector},
    graphics::{Background::Col, Color},
    input::{ButtonState, Key},
    lifecycle::{Event, Settings, State, Window, run},
};

const WIN_X: u32 = 800;
const WIN_Y: u32 = 600;

struct Conway {
    run: bool,
    elapsed: f64,
    steps: usize,
    map_x: usize,
    map_y: usize,
    state: Vec<bool>,
}

impl State for Conway {
    fn new() -> Result<Conway> {
        let filename = "map.txt";
        let map = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
        let (map_x, map_y) = find_dimensions(&map);
        let state = read_state(&map);
        Ok(Conway {run: true, elapsed: 0., steps: 0, state: state, map_x: map_x, map_y: map_y})
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        if let Event::Key(Key::Space, ButtonState::Pressed) = event {
            self.run = !self.run;
        }
        Ok(())
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if self.run {
            self.elapsed += window.update_rate();
            // run at 5fps
            while self.elapsed / 200. > self.steps as f64 {
                evolve_state(&mut self.state, self.map_x, self.map_y);
                self.steps += 1;
            }
        }
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let cell_width = WIN_X / self.map_x as u32;
        let cell_height = WIN_Y / self.map_y as u32;
        let mut cur_x = 0;
        let mut cur_y = 0;

        window.clear(Color::WHITE)?;

        for cell in &self.state {
            if *cell {
                window.draw(&Rectangle::new((cur_x, cur_y), (cell_width, cell_height)), Col(Color::BLUE));
            }
            cur_x += cell_width;
            if cur_x >= cell_width * self.map_x as u32 {
                cur_x = 0;
                cur_y += cell_height;
            }
        }

        Ok(())
    }
}

fn main() {
    run::<Conway>("Conway", Vector::new(WIN_X, WIN_Y), Settings::default());
}

fn find_dimensions(map: &str) -> (usize, usize) {
    let mut x_init = false;
    let mut x: usize = 0;
    let mut y: usize = 0;
    for line in map.lines() {
        y += 1;
        if !x_init {
            x = line.len();
            x_init = true;
        }
        else if x != line.len() {
            panic!("map width is not consistent");
        }
    }
    (x, y)
}

fn read_state(map: &str) -> Vec<bool> {
    let mut state: Vec<bool> = Vec::new();
    for line in map.lines() {
        for chr in line.chars() {
            if chr == '_' {
                state.push(false);
            }
            else if chr == '*' {
                state.push(true);
            }
            else {
                panic!("invalid char in map");
            }
        }
    }
    (state)
}

fn evolve_state(state: &mut Vec<bool>, map_x: usize, map_y: usize) {
    let mut transitions: Vec<(usize, bool)> = Vec::new();
    let mut cur_x: usize;
    let mut cur_y: usize;

    // create a list of transitions to apply
    for i in 0..state.len() {
        // convert vector index to coord
        cur_x = i % map_x;
        cur_y = i / map_x;
        // calculate number of live neighbors of this cell
        let mut live_neighbors = 0;
        // west
        if cur_x > 0 && state[i-1] {
            live_neighbors += 1;
        }
        // northwest
        if cur_x > 0 && cur_y > 0 && state[i-1-map_x] {
            live_neighbors += 1;
        }
        // north
        if cur_y > 0 && state[i-map_x] {
            live_neighbors += 1;
        }
        // northeast
        if cur_x < map_x-1 && cur_y > 0 && state[i+1-map_x] {
            live_neighbors += 1;
        }
        // east
        if cur_x < map_x-1 && cur_y < map_y-1 && state[i+1] {
            live_neighbors += 1;
        }
        // southeast
        if cur_x < map_x-1 && cur_y < map_y-1 && state[i+1+map_x] {
            live_neighbors += 1;
        }
        // south
        if cur_y < map_y-1 && state[i+map_x] {
            live_neighbors += 1;
        }
        // southwest
        if cur_x > 0 && cur_y < map_y-1 && state[i-1+map_x] {
            live_neighbors += 1;
        }
        // figure out next state
        if state[i] {
            if live_neighbors < 2 {
                // cell dies - under population
                transitions.push((i, false));
            }
            else if live_neighbors > 3 {
                // cell dies - over population
                transitions.push((i, false));
            }
        }
        else if live_neighbors == 3 {
            // cell is born - reproduction
            transitions.push((i, true));
        }
    }
    // apply transitions
    for (i, status) in transitions {
        state[i] = status;
    }
}
