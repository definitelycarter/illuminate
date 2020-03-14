use crate::message::Color;
use crate::proto::{Deserializable, Serializable};
use bytes::{Buf, BufMut, Bytes, BytesMut};

impl Serializable for Color {
  fn serialize(&self, bytes: &mut BytesMut) -> anyhow::Result<()> {
    bytes.put_u16_le(self.hue);
    bytes.put_u16_le(self.saturation);
    bytes.put_u16_le(self.brightness);
    bytes.put_u16_le(self.kelvin);
    Ok(())
  }
}

impl Deserializable for Color {
  fn deserialize(bytes: &mut Bytes) -> anyhow::Result<Self> {
    let hue = bytes.get_u16_le();
    let saturation = bytes.get_u16_le();
    let brightness = bytes.get_u16_le();
    let kelvin = bytes.get_u16_le();

    Ok(Self {
      hue,
      saturation,
      brightness,
      kelvin,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_serialize() {
    // using example found in the docs
    // https://lan.developer.lifx.com/docs/building-a-lifx-packet
    let color = Color::new(120, 100, 100, 3500);
    let mut bytes = BytesMut::new();
    color.serialize(&mut bytes).unwrap();
    assert_eq!(bytes, vec![0x55, 0x55, 0xff, 0xff, 0xff, 0xff, 0xac, 0xd])
  }
  #[test]
  fn test_deserialize() {
    // using example found in the docs
    // https://lan.developer.lifx.com/docs/building-a-lifx-packet
    let payload: &[u8] = &[0x55, 0x55, 0xff, 0xff, 0xff, 0xff, 0xac, 0xd];
    let mut bytes = Bytes::from(payload);
    let color = Color::deserialize(&mut bytes).unwrap();
    assert_eq!(color.hue(), 120);
    assert_eq!(color.brightness(), 100);
    assert_eq!(color.saturation(), 100);
    assert_eq!(color.kelvin(), 3500);
  }
}
