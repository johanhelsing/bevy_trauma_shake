use crate::Shake;
use bevy::{ecs::system::SystemParam, prelude::*};

/// Convenience [`SystemParam`] for adding trauma to all [`Shake`]s at the same
/// time.
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_trauma_shake::prelude::*;
/// fn add_shake(mut shakes: Shakes) {
///     shakes.add_trauma(0.2);
/// }
/// ```
#[derive(SystemParam)]
pub struct Shakes<'w, 's>(Query<'w, 's, &'static mut Shake>);

impl Shakes<'_, '_> {
    /// Adds specified trauma to all [`Shake`]s
    pub fn add_trauma(&mut self, trauma: f32) {
        for mut shake in &mut self.0 {
            shake.add_trauma(trauma);
        }
    }
}
