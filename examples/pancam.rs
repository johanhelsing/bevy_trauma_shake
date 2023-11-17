use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};
use bevy_trauma_shake::{prelude::*, TraumaPlugin};
use rand::prelude::random;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TraumaPlugin, PanCamPlugin::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, shake)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        PanCam::default(),
        Shake::default(),
    ));

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
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size,
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.),
                ..default()
            });
        }
    }
}

fn shake(mut shake: Query<&mut Shake>, keys: Res<Input<KeyCode>>) {
    let mut shake = shake.single_mut();

    if keys.just_pressed(KeyCode::Key1) {
        info!("Adding small trauma");
        shake.add_trauma(0.2);
    }

    if keys.just_pressed(KeyCode::Key2) {
        info!("Adding medium trauma");
        shake.add_trauma(0.5);
    }

    if keys.pressed(KeyCode::Key3) {
        info!("Sustaining max trauma");
        shake.add_trauma(1.);
    }
}
