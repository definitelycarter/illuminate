use crate::message::{Color, SetColorPayload, SetPowerPayload, StatePayload, StatePowerPayload};
use crate::proto::{Deserializable, Power, Serializable};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::convert::TryFrom;

impl Serializable for SetColorPayload {
  fn serialize(&self, bytes: &mut BytesMut) -> anyhow::Result<()> {
    // reserve u8;
    bytes.put_u8(0);
    self.color.serialize(bytes)?;
    bytes.put_u32_le(self.duration);
    Ok(())
  }
}

impl Deserializable for StatePayload {
  fn deserialize(bytes: &mut Bytes) -> anyhow::Result<Self> {
    let color = Color::deserialize(bytes)?;
    // skip 2 bytes
    bytes.advance(2);
    let power = bytes.get_u16_le();

    let mut label = [0_u8; 32];
    bytes.copy_to_slice(&mut label);

    // skip 2 bytes
    bytes.advance(8);
    Ok(Self {
      color,
      power,
      label,
    })
  }
}

impl Serializable for SetPowerPayload {
  fn serialize(&self, bytes: &mut BytesMut) -> anyhow::Result<()> {
    bytes.put_u16_le(self.level.into());
    bytes.put_u32_le(self.duration);
    Ok(())
  }
}

impl Deserializable for StatePowerPayload {
  fn deserialize(bytes: &mut Bytes) -> anyhow::Result<Self> {
    let level = bytes.get_u16_le();
    let level = Power::try_from(level)?;
    Ok(Self { level })
  }
}
