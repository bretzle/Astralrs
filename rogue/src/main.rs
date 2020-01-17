//! A Rogue-Like game built using the Fractal Engine Crate

#[macro_use]
extern crate specs_derive;

mod components;
mod map;
mod player;
mod rect;
mod visibility_system;

use crate::components::*;
use crate::map::*;
use crate::player::*;
use fractal::Fractal;
use fractal::GameState;
use fractal::*;
use specs::prelude::*;

pub struct State {
    pub ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Fractal) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

fn main() {
    let context = Fractal::init_simple(80, 50, "Hello Rust World", "resources");
    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    let map = Map::new_map();
    let (player_x, player_y) = map.rooms[0].center();
    gs.ecs.insert(map);

    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: fractal::to_keycode('@'),
            fg: color::YELLOW,
            bg: color::BLACK,
        })
        .with(Player {})
        .build();

    fractal::main_loop(context, gs);
}
