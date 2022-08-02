use crate::core::*;
use specs::prelude::*;
use crate::vitals;
use bracket_lib::prelude::*;
use crate::act;

pub struct State {
    world: World,
    player: Entity,
}

impl State {
    pub fn new() -> State {
        let mut world = World::new();
        world.register::<Pos>();
        world.register::<Draw>();
        world.register::<Player>();
        vitals::register_vitals(&mut world);

        let player =
            world.create_entity()
            .with(Player{})
            .with(Pos::new(0,0))
            .with(Draw{tile: "@".to_string()})
            .with(vitals::SpO2::default())
            .build();
        world.create_entity().with(Pos::new(1,1)).with(Draw{tile: "X".to_string()}).build();
        world.create_entity().with(Pos::new(2,4)).with(Draw{tile: "u".to_string()}).build();
        State {
            world,
            player,
        }
    }

    fn decode_input(&self, ctx: &mut BTerm) -> Option<act::Input> {
        use crate::act::Input::*;

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

    fn act_upon(&self, input: &act::Input) -> act::Action {
        use crate::act::Action::*;
        use crate::act::Input::*;
        let player = self.player;
        match input {
            Left => Move(player, Point::new(-1, 0)),
            Right => Move(player, Point::new(1, 0)),
            Up => Move(player, Point::new(0, -1)),
            Down => Move(player, Point::new(0, 1)),
            // _ => Nil,
        }
    }

    fn perform(&mut self, act: act::Action) {
        use act::Action::*;
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
        let tiles = self.world.read_storage::<Draw>();
        for (p, t) in (&positions, &tiles).join() {
            ctx.print(p.0.x, p.0.y, &t.tile);
        }
    }

}
