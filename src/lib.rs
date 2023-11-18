use bevy::{
    math::vec2,
    prelude::*,
    transform::systems::{propagate_transforms, sync_simple_transforms},
};
use noisy_bevy::fbm_simplex_2d;

#[cfg(feature = "commands")]
mod commands;

#[cfg(feature = "events")]
mod events;

#[cfg(feature = "system_param")]
mod system_param;

#[cfg(feature = "events")]
pub use events::TraumaEvent;

#[cfg(feature = "commands")]
pub use commands::TraumaCommands;

#[cfg(feature = "system_param")]
pub use system_param::Shakes;

pub mod prelude {
    #[cfg(feature = "system_param")]
    pub use super::Shakes;
    #[cfg(feature = "commands")]
    pub use super::TraumaCommands;
    #[cfg(feature = "events")]
    pub use super::TraumaEvent;
    pub use super::{Shake, ShakeSettings, TraumaPlugin};
}

pub struct TraumaPlugin;

impl Plugin for TraumaPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<Shake>()
            .register_type::<ShakeSettings>()
            .add_systems(PreUpdate, restore)
            .add_systems(
                PostUpdate,
                shake
                    .before(propagate_transforms)
                    .before(sync_simple_transforms),
            );

        #[cfg(feature = "events")]
        app.add_plugins(events::TraumaEventsPlugin);
    }
}

/// These can generally be left unchanged.
///
/// Optional configuration defaults will be used if not added
#[derive(Component, Reflect, Clone, Debug)]
pub struct ShakeSettings {
    /// the amplitude of the shake, how far it can offset
    pub amplitude: f32,
    /// normally in the 2-3 range, a high power makes low traumas less intense
    pub trauma_power: f32,
    /// how much trauma is reduced each second
    pub decay_per_second: f32,
    /// how frequently noise can change from minimum to maximum
    pub frequency: f32,
    /// how many layers of noise (detail if you will)
    pub octaves: usize,
}

impl Default for ShakeSettings {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl ShakeSettings {
    pub const DEFAULT: ShakeSettings = ShakeSettings {
        trauma_power: 2.,
        decay_per_second: 0.8,
        amplitude: 100.,
        frequency: 15.,
        octaves: 1,
    };
}

#[derive(Component, Reflect, Default, Clone, Debug)]
pub struct Shake {
    trauma: f32,
    reference_translation: Option<Vec3>,
}

impl Shake {
    pub fn add_trauma(&mut self, amount: f32) {
        self.trauma = (self.trauma + amount).clamp(0., 1.);
    }
}

#[derive(Component, Reflect, Clone, Copy)]
pub struct OriginalTranslation(pub Vec3);

fn shake(mut shakes: Query<(&mut Shake, &mut Transform, Option<&ShakeSettings>)>, time: Res<Time>) {
    for (mut shake, mut transform, settings) in &mut shakes {
        let settings = settings.unwrap_or(&ShakeSettings::DEFAULT);

        let trauma = f32::max(
            shake.trauma - settings.decay_per_second * time.delta_seconds(),
            0.0,
        );

        // avoid change detection
        if shake.trauma != trauma {
            shake.trauma = trauma;
        }

        let trauma_amount = f32::powf(shake.trauma, settings.trauma_power);

        if trauma_amount <= 0. {
            return;
        }

        shake.reference_translation = Some(transform.translation);

        let lacunarity = 2.;
        let gain = 0.5;
        let noise_pos = vec2(settings.frequency * time.elapsed_seconds(), 0.);
        let offset = settings.amplitude
            * trauma_amount
            * Vec2::new(
                fbm_simplex_2d(noise_pos + vec2(0., 1.), settings.octaves, lacunarity, gain),
                fbm_simplex_2d(noise_pos + vec2(0., 2.), settings.octaves, lacunarity, gain),
            );

        transform.translation.x += offset.x;
        transform.translation.y += offset.y;
    }
}

fn restore(mut shakes: Query<(&mut Shake, &mut Transform)>) {
    for (mut shake, mut transform) in &mut shakes {
        // avoid change detection
        if shake.reference_translation.is_some() {
            let translation = shake.reference_translation.take().unwrap();
            transform.translation = translation;
        }
    }
}
