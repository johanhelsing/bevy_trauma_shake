//! This example shows how to use the events-based trauma API
//!
//! Any `TraumaEvent`s sent will be applied to all cameras with the Shake
//! component on it.

use bevy::prelude::*;
use bevy_trauma_shake::prelude::*;
use rand::prelude::random;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TraumaPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, shake)
        .run();
}

fn setup(mut commands: Commands) {
    // Simply add a `Shake` component to your camera
    commands.spawn((Camera2d, Shake::default()));

    // generate some tiles to look at
    let n = 20;
    let spacing = 50.;
    let offset = spacing * n as f32 / 2.;
    let custom_size = Some(Vec2::new(spacing, spacing));
    for x in 0..n {
        for y in 0..n {
            let x = x as f32 * spacing - offset;
            let y = y as f32 * spacing - offset;
            let color = Color::hsl(240., random::<f32>() * 0.3, random::<f32>() * 0.3);
            commands.spawn((
                Sprite {
                    color,
                    custom_size,
                    ..default()
                },
                Transform::from_xyz(x, y, 0.),
            ));
        }
    }
}

fn shake(mut trauma: EventWriter<TraumaEvent>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Digit1) {
        info!("Adding small trauma");
        trauma.send(0.2.into());
    }

    if keys.just_pressed(KeyCode::Digit2) {
        info!("Adding medium trauma");
        trauma.send(TraumaEvent(0.5));
    }

    if keys.pressed(KeyCode::Digit3) {
        info!("Sustaining max trauma");
        trauma.send(TraumaEvent::MAX);
    }
}
