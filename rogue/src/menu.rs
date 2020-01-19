use crate::gui::*;
use crate::RunState;
use crate::State;
use fractal::color;
use fractal::console::Console;
use fractal::fractal::Fractal;
use fractal::VirtualKeyCode;

pub fn main_menu(gs: &mut State, ctx: &mut Fractal) -> MainMenuResult {
    let runstate = gs.ecs.fetch::<RunState>();

    ctx.print_color_centered(15, color::YELLOW, color::BLACK, "Rust Roguelike Tutorial");

    if let RunState::MainMenu {
        menu_selection: selection,
    } = *runstate
    {
        if selection == MainMenuSelection::NewGame {
            ctx.print_color_centered(24, color::MAGENTA, color::BLACK, "Begin New Game");
        } else {
            ctx.print_color_centered(24, color::WHITE, color::BLACK, "Begin New Game");
        }

        if selection == MainMenuSelection::LoadGame {
            ctx.print_color_centered(25, color::MAGENTA, color::BLACK, "Load Game");
        } else {
            ctx.print_color_centered(25, color::WHITE, color::BLACK, "Load Game");
        }

        if selection == MainMenuSelection::Quit {
            ctx.print_color_centered(26, color::MAGENTA, color::BLACK, "Quit");
        } else {
            ctx.print_color_centered(26, color::WHITE, color::BLACK, "Quit");
        }

        match ctx.key {
            None => {
                return MainMenuResult::NoSelection {
                    selected: selection,
                }
            }
            Some(key) => match key {
                VirtualKeyCode::Escape => {
                    return MainMenuResult::NoSelection {
                        selected: MainMenuSelection::Quit,
                    }
                }
                VirtualKeyCode::Up => {
                    let newselection;
                    match selection {
                        MainMenuSelection::NewGame => newselection = MainMenuSelection::Quit,
                        MainMenuSelection::LoadGame => newselection = MainMenuSelection::NewGame,
                        MainMenuSelection::Quit => newselection = MainMenuSelection::LoadGame,
                    }
                    return MainMenuResult::NoSelection {
                        selected: newselection,
                    };
                }
                VirtualKeyCode::Down => {
                    let newselection;
                    match selection {
                        MainMenuSelection::NewGame => newselection = MainMenuSelection::LoadGame,
                        MainMenuSelection::LoadGame => newselection = MainMenuSelection::Quit,
                        MainMenuSelection::Quit => newselection = MainMenuSelection::NewGame,
                    }
                    return MainMenuResult::NoSelection {
                        selected: newselection,
                    };
                }
                VirtualKeyCode::Return => {
                    return MainMenuResult::Selected {
                        selected: selection,
                    }
                }
                _ => {
                    return MainMenuResult::NoSelection {
                        selected: selection,
                    }
                }
            },
        }
    }

    MainMenuResult::NoSelection {
        selected: MainMenuSelection::NewGame,
    }
}
