use super::color::Color;
use crate::proto::Power;

pub struct SetColorPayload {
  pub(crate) color: Color,
  pub(crate) duration: u32,
}

pub struct SetWaveformPayload {
  // reserve u8
  pub(crate) transient: i8, // maybe this is a u8?
  pub(crate) color: Color,
  pub(crate) period: u32,
  pub(crate) cycles: f32,
  pub(crate) skew_ration: i16,
  pub(crate) waveform: u8,
}

pub struct StatePayload {
  pub color: Color,
  pub power: u16,
  pub label: [u8; 32],
}

impl StatePayload {
  pub fn power(&self) -> u16 {
    let power = self.power as f32;
    (power / 65535_f32 * 100_f32).ceil() as u16
  }
}

pub struct SetPowerPayload {
  pub(crate) level: Power,
  pub(crate) duration: u32,
}

pub struct StatePowerPayload {
  pub level: Power,
}

pub struct InfraredPayload {
  // this is the brightness playa
  pub(crate) brightness: u16,
}
