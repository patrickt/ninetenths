use bracket_lib::prelude::*;

struct State {
}

impl State {
    fn new() -> State {
        State {}
    }

}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "possession is nine-tenths of the law");
    }

}

fn run() -> BError {
    let bterm = BTermBuilder::simple80x50().with_title("ninetenths").build()?;
    main_loop(bterm, State::new())
}

fn main() {
    println!("Hello, world!");
    run().unwrap();
}
