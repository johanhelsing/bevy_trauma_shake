use crate::Shake;
use bevy::prelude::*;

pub struct TraumaEventsPlugin;

impl Plugin for TraumaEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(apply_trauma_event);
    }
}

/// Event for adding trauma to all shakes
///
/// Trigger this event using `Commands`:
/// ```
/// # use bevy::prelude::*;
/// # use bevy_trauma_shake::prelude::*;
/// fn add_shake(mut commands: Commands) {
///     commands.trigger(TraumaEvent(0.2));
/// }
/// ```
#[derive(Event, Debug, Clone, Copy, Reflect)]
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

fn apply_trauma_event(trigger: On<TraumaEvent>, mut shakes: Query<&mut Shake>) {
    let trauma = trigger.event().0;

    if trauma > 0. {
        for mut shake in &mut shakes {
            shake.add_trauma(trauma);
        }
    }
}
