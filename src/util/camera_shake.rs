use bevy::math::vec2;
use bevy::prelude::*;
use noisy_bevy::fbm_simplex_2d;

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

#[derive(Component, Reflect, Default, Clone, Debug)]
pub struct Shake {
    trauma: f32,
    reference_translation: Option<Vec3>,
}

impl Shake {
    /// Adds the specified trauma. Trauma is clamped between 0 and 1, and decays
    /// over time according to [`ShakeSettings::decay_per_second`].
    pub fn add_trauma(&mut self, amount: f32) {
        self.trauma = (self.trauma + amount).clamp(0., 1.);
    }
}

pub(crate) fn apply_trauma_events(
    mut events: EventReader<TraumaEvent>,
    mut shakes: Query<&mut Shake>,
) {
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
    const DEFAULT: ShakeSettings = ShakeSettings {
        trauma_power: 3.,
        decay_per_second: 0.8,
        amplitude: 2.,
        frequency: 12.,
        octaves: 2,
    };
}

/// Makes the entity shake according to applied trauma.
///
/// The shake happens during [`PostUpdate`], and the entity is restored to its
/// original translation in [`PreUpdate`]. This means that you can still control
/// the camera like you normally would inside update.

pub fn shake(
    mut shakes: Query<(&mut Shake, &mut Transform, Option<&ShakeSettings>)>,
    time: Res<Time>,
) {
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

pub(crate) fn restore(mut shakes: Query<(&mut Shake, &mut Transform)>) {
    for (mut shake, mut transform) in &mut shakes {
        // avoid change detection
        if shake.reference_translation.is_some() {
            let translation = shake.reference_translation.take().unwrap();
            transform.translation = translation;
        }
    }
}
