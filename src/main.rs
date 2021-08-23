use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End,
}

struct State {
    mode: GameMode,
}

impl State {
    pub fn new() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut bracket_lib::prelude::BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello first game");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    let state = State::new();
    main_loop(context, state)
}
