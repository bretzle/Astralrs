//! A Rogue-Like game built using the Fractal Engine Crate

#[macro_use]
extern crate specs_derive;

mod components;
mod map;
mod monster_ai_system;
mod player;
mod rect;
mod visibility_system;

use crate::components::*;
use crate::map::*;
use crate::monster_ai_system::MonsterAI;
use crate::player::*;
use crate::visibility_system::VisibilitySystem;
use fractal::codepage437::to_cp437;
use fractal::color;
use fractal::console::Console;
use fractal::fractal::main_loop;
use fractal::fractal::Fractal;
use fractal::random::RandomNumberGenerator;
use fractal::GameState;
use specs::prelude::*;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    Running,
}

pub struct State {
    pub ecs: World,
    pub runstate: RunState,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAI {};
        mob.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Fractal) {
        ctx.cls();

        if self.runstate == RunState::Running {
            self.run_systems();
            self.runstate = RunState::Paused;
        } else {
            self.runstate = player_input(self, ctx);
        }

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
    }
}

fn main() {
    let context = Fractal::init_simple8x8(80, 50, "Hello Rust World", "resources");
    let mut gs = State {
        ecs: World::new(),
        runstate: RunState::Running,
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Viewshed>();

    let map: Map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    let mut rng = RandomNumberGenerator::new();
    for room in map.rooms.iter().skip(1) {
        let (x, y) = room.center();

        let glyph: u8;
        let roll = rng.roll_dice(1, 2);

        match roll {
            1 => glyph = to_cp437('g'),
            _ => glyph = to_cp437('c'),
        }

        gs.ecs
            .create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph: glyph,
                fg: color::RED,
                bg: color::BLACK,
            })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .with(Monster {})
            .build();
    }

    gs.ecs.insert(map);

    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: color::YELLOW,
            bg: color::BLACK,
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 10,
            dirty: true,
        })
        .build();

    main_loop(context, gs);
}
