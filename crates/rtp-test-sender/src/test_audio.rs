#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]

use anyhow::Result;
use std::f32::consts::PI;
use tracing::info;

pub struct AudioGenerator {
    sample_rate: u32,
    channels: u8,
    frequency: f32,
    phase: f32,
    swapped: bool,
}

impl AudioGenerator {
    pub fn new(sample_rate: u32, channels: u8) -> Self {
        Self {
            sample_rate,
            channels,
            frequency: 440.0, // A4 tone
            phase: 0.0,
            swapped: false,
        }
    }

    pub fn generate_samples(&mut self, num_samples: usize) -> Vec<u8> {
        let mut samples = Vec::with_capacity(num_samples * self.channels as usize);

        for _ in 0..num_samples {
            let sample = (self.phase.sin() * 0.5 * f32::from(i16::MAX)) as i16;

            if self.channels == 2 {
                // For stereo, generate different frequencies for each channel
                let left_sample = sample;
                let right_sample = ((self.phase * 1.5).sin() * 0.3 * f32::from(i16::MAX)) as i16;

                if self.swapped {
                    // Swap channels
                    samples.extend_from_slice(&right_sample.to_le_bytes());
                    samples.extend_from_slice(&left_sample.to_le_bytes());
                } else {
                    samples.extend_from_slice(&left_sample.to_le_bytes());
                    samples.extend_from_slice(&right_sample.to_le_bytes());
                }
            } else {
                // Mono
                samples.extend_from_slice(&sample.to_le_bytes());
            }

            self.phase += 2.0 * PI * self.frequency / self.sample_rate as f32;
            if self.phase > 2.0 * PI {
                self.phase -= 2.0 * PI;
            }
        }

        samples
    }

    pub fn swap_channels(&mut self) {
        if self.channels == 2 {
            self.swapped = !self.swapped;
            info!("Channels swapped: {}", self.swapped);
        }
    }

    pub fn generate_pcmu_samples(&mut self, num_samples: usize) -> Vec<u8> {
        let mut samples = Vec::with_capacity(num_samples);

        for _ in 0..num_samples {
            let sample = (self.phase.sin() * 0.5 * f32::from(i16::MAX)) as i16;
            let pcmu = linear_to_ulaw(sample);
            samples.push(pcmu);

            self.phase += 2.0 * PI * self.frequency / self.sample_rate as f32;
            if self.phase > 2.0 * PI {
                self.phase -= 2.0 * PI;
            }
        }

        samples
    }
}

// µ-law encoding for G.711
fn linear_to_ulaw(sample: i16) -> u8 {
    const BIAS: i16 = 0x84;
    const CLIP: i16 = 32635;

    let mut s = sample >> 2;
    if s < 0 {
        s = -s;
    }
    if s > CLIP {
        s = CLIP;
    }
    s += BIAS;

    let exponent = s.leading_zeros() as i16 - 17;
    let mantissa = (s >> (exponent + 3)) & 0x0F;
    let mut ulawbyte = !(mantissa | (exponent << 4));

    if sample < 0 {
        ulawbyte &= 0x7F;
    }

    ulawbyte as u8
}

#[allow(dead_code)]
#[allow(clippy::unnecessary_wraps)]
pub fn load_audio_file(_path: &str) -> Result<Vec<u8>> {
    // TODO: Implement actual WAV file loading with hound
    // For now, return empty to use generated audio
    Ok(Vec::new())
}
