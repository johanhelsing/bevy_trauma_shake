use crate::Shake;
use bevy::{ecs::system::Command, prelude::*};

pub trait TraumaCommands {
    /// Applies the given trauma to all `Shake`s
    fn add_trauma(&mut self, trauma: f32);
}

impl TraumaCommands for Commands<'_, '_> {
    fn add_trauma(&mut self, trauma: f32) {
        self.add(AddTraumaCommand(trauma));
    }
}

struct AddTraumaCommand(f32);

impl Command for AddTraumaCommand {
    fn apply(self, world: &mut World) {
        for mut shake in world.query::<&mut Shake>().iter_mut(world) {
            shake.add_trauma(self.0);
        }
    }
}
