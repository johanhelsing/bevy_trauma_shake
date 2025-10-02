use crate::{shake, Shake};
use bevy::prelude::*;

pub struct TraumaEventsPlugin;

impl Plugin for TraumaEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<TraumaEvent>()
            .add_systems(PostUpdate, apply_trauma_events.before(shake));
    }
}

/// Message for adding trauma to all shakes
/// ```
/// # use bevy::prelude::*;
/// # use bevy_trauma_shake::prelude::*;
/// fn add_shake(mut trauma: MessageWriter<TraumaEvent>) {
///     trauma.write(0.2.into());
/// }
/// ```
#[derive(Message, Debug, Clone, Copy, Reflect)]
pub struct TraumaEvent(pub f32);

impl From<f32> for TraumaEvent {
    fn from(value: f32) -> Self {
        TraumaEvent(value)
    }
}

impl TraumaEvent {
    /// Maximum (1) trauma
    pub const MAX: TraumaEvent = TraumaEvent(1.);
}

fn apply_trauma_events(mut events: MessageReader<TraumaEvent>, mut shakes: Query<&mut Shake>) {
    let mut trauma = 0.;

    for event in events.read() {
        trauma += event.0;
    }

    if trauma > 0. {
        for mut shake in &mut shakes {
            shake.add_trauma(trauma);
        }
    }
}
