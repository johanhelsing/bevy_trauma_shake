use crate::Shake;
use bevy::{ecs::system::SystemParam, prelude::*};

#[derive(SystemParam)]
pub struct Shakes<'w, 's>(Query<'w, 's, &'static mut Shake>);

impl<'w, 's> Shakes<'w, 's> {
    pub fn add_trauma(&mut self, trauma: f32) {
        for mut shake in &mut self.0 {
            shake.add_trauma(trauma);
        }
    }
}
