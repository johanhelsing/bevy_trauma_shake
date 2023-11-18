use crate::{shake, Shake};
use bevy::prelude::*;

pub struct TraumaEventsPlugin;

impl Plugin for TraumaEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TraumaEvent>()
            .add_systems(PostUpdate, apply_trauma_events.before(shake));
    }
}

#[derive(Event, Debug, Clone, Copy, Reflect)]
pub struct TraumaEvent(pub f32);

impl From<f32> for TraumaEvent {
    fn from(value: f32) -> Self {
        TraumaEvent(value)
    }
}

impl TraumaEvent {
    pub const MAX: TraumaEvent = TraumaEvent(1.);
}

fn apply_trauma_events(mut events: EventReader<TraumaEvent>, mut shakes: Query<&mut Shake>) {
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
