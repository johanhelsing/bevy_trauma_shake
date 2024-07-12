use crate::Shake;
use bevy::{ecs::world::Command, prelude::*};

/// Extension trait for [`Command`], adding commands for easily applying trauma
/// fire-and-forget-style.
pub trait TraumaCommands {
    /// Applies the given trauma to all `Shake`s
    /// ```
    /// # use bevy::prelude::*;
    /// use bevy_trauma_shake::prelude::*;
    ///
    /// fn add_shake(mut commands: Commands) {
    ///     commands.add_trauma(0.2);
    /// }
    /// ```
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
