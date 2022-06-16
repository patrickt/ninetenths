use bracket_lib::prelude::*;
use specs::prelude::*;
use specs_derive::Component;
use derive_more::Display;

struct State {
    world: World,
    player: Entity,
}

#[derive(Component)]
struct Pos(Point);

#[derive(Component)]
struct Tile {
    tile: String,
}

#[derive(Component)]
struct Player {}

enum Input {
    Left, Right, Up, Down,
}

enum Action {
    Nil,
    Move(Entity, Point),
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
        world.register::<Player>();
        let player = world.create_entity().with(Player{}).with(Pos::new(0,0)).with(Tile{tile: "@".to_string()}).build();
        world.create_entity().with(Pos::new(1,1)).with(Tile{tile: "X".to_string()}).build();
        world.create_entity().with(Pos::new(2,4)).with(Tile{tile: "u".to_string()}).build();
        State {
            world,
            player,
        }
    }

    fn decode_input(&self, ctx: &mut BTerm) -> Option<Input> {
        use Input::*;

        match ctx.key {
            None => None,
            Some(s) => match s {
                VirtualKeyCode::Down => Some(Down),
                VirtualKeyCode::Right => Some(Right),
                VirtualKeyCode::Up => Some(Up),
                VirtualKeyCode::Left => Some(Left),
                _ => None,
            }
        }
    }

    fn act_upon(&self, input: &Input) -> Action {
        use Action::*;
        use Input::*;
        let player = self.player;
        match input {
            Left => Move(player, Point::new(-1, 0)),
            Right => Move(player, Point::new(1, 0)),
            Up => Move(player, Point::new(0, -1)),
            Down => Move(player, Point::new(0, 1)),
            // _ => Nil,
        }
    }

    fn perform(&mut self, act: Action) {
        use Action::*;
        let mut positions = self.world.write_storage::<Pos>();
        match act {
            Move(e, p) => {
                if let Some(player) = positions.get_mut(e) {
                    player.0 += p;
                }
            }
            Nil => {}
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        if let Some(i) = self.decode_input(ctx) {
            self.perform(self.act_upon(&i))
        }

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
