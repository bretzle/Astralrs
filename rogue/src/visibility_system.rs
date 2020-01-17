use crate::components::Player;
use crate::components::Position;
use crate::components::Viewshed;
use crate::geometry::point::Point;
use crate::map::Map;
use fractal::field_of_view;
use specs::prelude::*;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, pos, player) = data;

        for (entity, view, pos) in (&entities, &mut viewshed, &pos).join() {
            if view.dirty {
                view.dirty = false;
                view.visible_tiles.clear();
                view.visible_tiles = field_of_view(Point::new(pos.x, pos.y), view.range, &*map);
                view.visible_tiles
                    .retain(|p| p.x > 0 && p.x < map.width - 1 && p.y > 0 && p.y < map.height - 1);

                // if this is a player, reveal what they can see
                let p = player.get(entity);
                if let Some(p) = p {
                    for t in map.visible_tiles.iter_mut() {
                        *t = false
                    }
                    for vis in view.visible_tiles.iter() {
                        let idx = map.xy_idx(vis.x, vis.y);
                        map.revealed_tiles[idx] = true;
                        map.visible_tiles[idx] = true;
                    }
                }
            }
        }
    }
}
