//! A Rogue-Like game built using the Fractal Engine Crate

use fractal::color::{self, Color};
use fractal::{Console, Fractal, GameState};
use specs::prelude::*;

#[macro_use]
extern crate specs_derive;

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: u8,
    fg: Color,
    bg: Color,
}

struct State {
    ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Fractal) {
        ctx.cls();
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() {
    let context = Fractal::init_simple(80, 50, "Hello Rust World", "resources");
    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();

    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: fractal::to_keycode('@'),
            fg: color::YELLOW,
            bg: color::BLACK,
        })
        .build();

    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(Position { x: i * 7, y: 20 })
            .with(Renderable {
                glyph: fractal::to_keycode('☺'),
                fg: color::RED,
                bg: color::BLACK,
            })
            .build();
    }

    fractal::main_loop(context, gs);
}
