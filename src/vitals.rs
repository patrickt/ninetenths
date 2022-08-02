use specs::prelude::*;
use specs_derive::Component;
use crate::dhall::raw;
use crate::core::*;

#[derive(Component)]
pub struct SpO2(f32);

impl SpO2 {
    pub fn is_hypoxia(&self) -> bool {
        self.0 > 0.9
    }

    pub fn incr(&mut self) {
        self.0 += 0.01;
        self.0 = self.0.max(1.0)
    }

    pub fn decr(&mut self) {
        self.0 -= 0.01;
        self.0 = self.0.max(0.0)
    }
}

impl Default for SpO2 {
    fn default() -> SpO2 { SpO2(0.95) }
}

pub fn register_vitals(world: &mut World) {
    world.register::<SpO2>();
}

#[derive(Component)]
pub struct Organs(pub Vec<Handle<raw::BodyPart>>);

struct DecreaseO2<'a> {
    vault: &'a crate::vault::Vault
}

impl <'a, 't> System<'a> for DecreaseO2<'t> {
    type SystemData = (WriteStorage<'a, SpO2>, ReadStorage<'a, Organs>);

    fn run(&mut self, (mut o2, organs): Self::SystemData) {
        for (o2, organs) in (&mut o2, &organs).join() {
            for org in &organs.0 {
                match self.vault.get_body_part(*org).purpose {
                    raw::BioSystem::Respiratory => o2.incr(),
                    raw::BioSystem::Tissue => {},
                    _ => o2.decr(),
                }
            }
        }
    }
}
