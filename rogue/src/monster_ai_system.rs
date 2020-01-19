use crate::components::*;
use crate::map::Map;
use fractal::geometry::DistanceAlg::Pythagoras;
use fractal::geometry::Point;
use fractal::log;
use fractal::pathfinding::astar::a_star_search;
use specs::prelude::*;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, Point>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, player_pos, mut viewshed, monster, name, mut position) = data;

        for (mut viewshed, _monster, name, mut pos) in
            (&mut viewshed, &monster, &name, &mut position).join()
        {
            if viewshed.visible_tiles.contains(&*player_pos) {
                let path = a_star_search(
                    map.xy_idx(pos.x, pos.y) as i32,
                    map.xy_idx(player_pos.x, player_pos.y) as i32,
                    &mut *map,
                );
                let distance = Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
                if path.success && path.steps.len() > 1 && distance > 1.5 {
                    pos.x = path.steps[1] % map.width;
                    pos.y = path.steps[1] / map.width;
                    viewshed.dirty = true;
                }
                if distance < 1.5 {
                    log(&format!("{} shouts insults", name.name));
                    return;
                }
            }
        }
    }
}
