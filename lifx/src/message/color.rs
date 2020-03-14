use std::fmt;

pub struct Color {
  pub(crate) hue: u16,
  pub(crate) saturation: u16,
  pub(crate) brightness: u16,
  pub(crate) kelvin: u16,
}

impl Color {
  pub fn new(hue: u16, saturation: u16, brightness: u16, kelvin: u16) -> Color {
    Color {
      hue: Color::calc_hue(hue),
      saturation: Color::calc_saturation(saturation),
      brightness: Color::calc_brightness(brightness),
      kelvin,
    }
  }
  pub fn set_hue(&mut self, hue: u16) {
    self.hue = Color::calc_hue(hue);
  }
  pub fn hue(&self) -> u16 {
    let hue = self.hue as f32;
    (hue * 360_f32 / 65535_f32).ceil() as u16
  }

  pub fn set_saturation(&mut self, saturation: u16) {
    self.saturation = Color::calc_saturation(saturation);
  }
  pub fn saturation(&self) -> u16 {
    let saturation = self.saturation as f32;
    (saturation * 100_f32 / 65535_f32).ceil() as u16
  }

  pub fn set_brightness(&mut self, brightness: u16) {
    self.brightness = Color::calc_brightness(brightness);
  }
  pub fn brightness(&self) -> u16 {
    let brightness = self.brightness as f32;
    (brightness * 100_f32 / 65535_f32).ceil() as u16
  }

  pub fn set_kelvin(&mut self, kelvin: u16) {
    self.kelvin = kelvin
  }
  pub fn kelvin(&self) -> u16 {
    self.kelvin
  }

  fn calc_hue(hue: u16) -> u16 {
    let hue = hue as f32;
    (hue / 360_f32 * 65535_f32) as u16
  }
  fn calc_saturation(saturation: u16) -> u16 {
    let saturation = saturation as f32;
    (saturation / 100_f32 * 65535_f32) as u16
  }
  fn calc_brightness(brightness: u16) -> u16 {
    let brightness = brightness as f32;
    (brightness / 100_f32 * 65535_f32) as u16
  }
}

impl fmt::Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{},{},{},{}",
      self.hue(),
      self.saturation(),
      self.brightness(),
      self.kelvin()
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn should_calc_hue() {
    assert_eq!(Color::calc_hue(120), 21845);
  }
  #[test]
  fn should_return_color_values() {
    let color = Color::new(125, 99, 99, 4000);
    assert_eq!(color.hue(), 125);
    assert_eq!(color.saturation(), 99);
    assert_eq!(color.brightness(), 99);
    assert_eq!(color.kelvin(), 4000);

    let color = Color::new(74, 24, 24, 9000);
    assert_eq!(color.hue(), 74);
    assert_eq!(color.saturation(), 24);
    assert_eq!(color.brightness(), 24);
    assert_eq!(color.kelvin(), 9000);
  }
}
