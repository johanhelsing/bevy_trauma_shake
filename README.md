# bevy_trauma_shake

[![crates.io](https://img.shields.io/crates/v/bevy_trauma_shake.svg)](https://crates.io/crates/bevy_trauma_shake)
![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)
[![crates.io](https://img.shields.io/crates/d/bevy_trauma_shake.svg)](https://crates.io/crates/bevy_trauma_shake)
[![docs.rs](https://img.shields.io/docsrs/bevy_trauma_shake)](https://docs.rs/bevy_trauma_shake)

Add camera shakes to your 2d Bevy game with three lines of code.

## Usage

Add the plugin:

```rust
app.add_plugins(TraumaPlugin);
```

Simply add a component to your camera:

```rust
commands.spawn((Camera2dBundle::default(), Shake::default()));
```

Make it shake:

```rust
fn shake(mut shake: Query<&mut Shake>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        shake.single_mut().add_trauma(0.2);
    }
}
```

## Optional configuration

Optionally add `ShakeSettings`, if you're not happy with the defaults.

```rust
    commands.spawn((
        Name::new("Camera"),
        Camera2dBundle::default(),
        Shake::default(),
        ShakeSettings {
            amplitude: 200.,
            trauma_power: 3.,
            decay_per_second: 0.3,
            frequency: 4.,
            octaves: 2,
        },
        PanCam::default(),
    ));
```

## Goals

- Minimal dependencies
- Zero configuration required
- Sensible defaults
- Batteries included (default noise)
- Works with bevy_pancam

## Bevy Version Support

The `main` branch targets the latest bevy release.

|bevy|bevy_trauma_shake|
|----|-----------------|
|0.12|main             |

## License

`bevy_trauma_shake` is dual-licensed under either

- MIT License (./LICENSE-MIT or <http://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 (./LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)

at your option.

## Thanks

- <https://www.youtube.com/watch?v=tu-Qe66AvtY>
- [`bevy_camera_shake`](https://github.com/Andrewp2/bevy_camera_shake): 2D and 3D shakes and more configuration options. I used this a lot for reference, but I wanted a simpler API.

## Contributions

PRs welcome!
