//! A Rogue-Like game built using the Fractal Engine Crate

#[macro_use]
extern crate specs_derive;

use fractal::GameState;
use specs::prelude::*;

struct State {
    ecs: World,
}

impl GameState for State {
    fn tick(&mut self) {}
}

fn main() {
    println!("Hello, world!");
}
