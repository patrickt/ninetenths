use bracket_lib::prelude::*;
use specs::prelude::*;
use specs_derive::Component;
use derive_more::Display;

struct State {
    world: World,
}

#[derive(Component)]
struct Pos(Point);

#[derive(Component)]
struct Tile {
    tile: String,
}

impl Pos {
    fn new(x: i32, y: i32) -> Pos {
        Pos(Point{x,y})
    }
}

impl State {
    fn new() -> State {
        let mut world = World::new();
        world.register::<Pos>();
        world.register::<Tile>();
        world.create_entity().with(Pos::new(0,0)).with(Tile{tile: "@".to_string()}).build();
        world.create_entity().with(Pos::new(1,1)).with(Tile{tile: "X".to_string()}).build();
        State {
            world,
        }
    }

}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        let positions = self.world.read_storage::<Pos>();
        let tiles = self.world.read_storage::<Tile>();
        for (p, t) in (&positions, &tiles).join() {
            ctx.print(p.0.x, p.0.y, &t.tile);
        }
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
