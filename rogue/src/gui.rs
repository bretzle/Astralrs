use crate::components::*;
use fractal::color;
use fractal::console::Console;
use fractal::fractal::Fractal;
use specs::prelude::*;

pub fn draw_ui(ecs: &World, ctx: &mut Fractal) {
    ctx.draw_box(0, 43, 79, 6, color::WHITE, color::BLACK);

    let combat_stats = ecs.read_storage::<CombatStats>();
    let players = ecs.read_storage::<Player>();
    for (_player, stats) in (&players, &combat_stats).join() {
        let health = format!(" HP: {} / {} ", stats.hp, stats.max_hp);
        ctx.print_color(12, 43, color::YELLOW, color::BLACK, &health);

        ctx.draw_bar_horizontal(28, 43, 51, stats.hp, stats.max_hp, color::RED, color::BLACK);
    }
}
