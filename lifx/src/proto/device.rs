use crate::message::{
  EchoPayload, FirmwarePayload, GroupPayload, LocationPayload, StateHostInfoPayload,
  StateInfoPayload, StateServicePayload, StateVersionPayload, StateWifiInfoPayload,
};
use crate::proto::{Deserializable, Serializable};
use bytes::{Buf, BufMut, Bytes, BytesMut};

impl Deserializable for StateServicePayload {
  fn deserialize(bytes: &mut Bytes) -> anyhow::Result<Self> {
    let service = bytes.get_u8();
    let port = bytes.get_u32_le();
    Ok(Self { service, port })
  }
}

impl Deserializable for StateHostInfoPayload {
  fn deserialize(bytes: &mut Bytes) -> anyhow::Result<Self> {
    let signal = bytes.get_f32_le();
    let tx = bytes.get_u32_le();
    let rx = bytes.get_u32_le();
    Ok(Self { signal, tx, rx })
  }
}

impl Deserializable for FirmwarePayload {
  fn deserialize(bytes: &mut Bytes) -> anyhow::Result<Self> {
    let build = bytes.get_u64_le();
    // skip 8 bytes
    bytes.advance(8);
    let version_minor = bytes.get_u16_le();
    let version_major = bytes.get_u16_le();
    Ok(Self {
      build,
      version_minor,
      version_major,
    })
  }
}

impl Serializable for FirmwarePayload {
  fn serialize(&self, bytes: &mut BytesMut) -> anyhow::Result<()> {
    bytes.put_u64_le(self.build);

    for _ in 0..8 {
      bytes.put_u8(0);
    }

    bytes.put_u16_le(self.version_minor);
    bytes.put_u16_le(self.version_major);
    Ok(())
  }
}

impl Deserializable for StateWifiInfoPayload {
  fn deserialize(bytes: &mut Bytes) -> anyhow::Result<Self> {
    let signal = bytes.get_f32_le();
    let tx = bytes.get_u32_le();
    let rx = bytes.get_u32_le();
    Ok(Self { signal, tx, rx })
  }
}

impl Deserializable for StateVersionPayload {
  fn deserialize(bytes: &mut Bytes) -> anyhow::Result<Self> {
    let vendor = bytes.get_u32_le();
    let product = bytes.get_u32_le();
    let version = bytes.get_u32_le();
    Ok(Self {
      vendor,
      product,
      version,
    })
  }
}

impl Deserializable for StateInfoPayload {
  fn deserialize(bytes: &mut Bytes) -> anyhow::Result<Self> {
    let time = bytes.get_u64_le();
    let uptime = bytes.get_u64_le();
    let downtime = bytes.get_u64_le();
    Ok(Self {
      time,
      uptime,
      downtime,
    })
  }
}

impl Deserializable for LocationPayload {
  fn deserialize(bytes: &mut Bytes) -> anyhow::Result<Self> {
    let mut location = [0_u8; 16];
    bytes.copy_to_slice(&mut location);

    let mut label = [0_u8; 32];
    bytes.copy_to_slice(&mut label);

    let updated_at = bytes.get_u64_le();

    Ok(Self {
      location,
      label,
      updated_at,
    })
  }
}

impl Deserializable for GroupPayload {
  fn deserialize(bytes: &mut Bytes) -> anyhow::Result<Self> {
    let mut group = [0_u8; 16];
    bytes.copy_to_slice(&mut group);

    let mut label = [0_u8; 32];
    bytes.copy_to_slice(&mut label);

    // todo - docs say i64??
    let updated_at = bytes.get_u64_le();

    Ok(Self {
      group,
      label,
      updated_at,
    })
  }
}

impl Serializable for GroupPayload {
  fn serialize(&self, bytes: &mut BytesMut) -> anyhow::Result<()> {
    bytes.copy_from_slice(&self.group);
    bytes.copy_from_slice(&self.label);
    bytes.put_u64_le(self.updated_at);
    Ok(())
  }
}

impl Deserializable for EchoPayload {
  fn deserialize(bytes: &mut Bytes) -> anyhow::Result<Self> {
    let mut payload = [0_u8; 64];
    bytes.copy_to_slice(&mut payload);

    Ok(Self { payload })
  }
}

impl Serializable for EchoPayload {
  fn serialize(&self, bytes: &mut BytesMut) -> anyhow::Result<()> {
    bytes.copy_from_slice(&self.payload);
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_state_service_serialize() {
    let payload = FirmwarePayload {
      build: 1902390209,
      version_minor: 55,
      version_major: 55,
    };
    let mut bytes = BytesMut::new();
    payload.serialize(&mut bytes).unwrap();
    eprintln!("{:x?}", bytes.to_vec());
    assert_eq!(
      bytes,
      vec![
        0xc1, 0x2b, 0x64, 0x71, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x37,
        0x0, 0x37, 0x0
      ]
    );
  }
  #[test]
  fn test_state_service_deserialize() {
    let payload: &[u8] = &[0x1, 0x7c, 0xdd, 0x0, 0x0];
    let mut bytes = Bytes::from(payload);
    let deserialized = StateServicePayload::deserialize(&mut bytes).unwrap();
    assert_eq!(deserialized.service, 1);
    assert_eq!(deserialized.port, 56700);
  }
  #[test]
  fn test_state_host_info_deserialize() {
    let payload: &[u8] = &[
      0xe7, 0xfb, 0x9, 0x3f, 0x37, 0x0, 0x0, 0x0, 0x37, 0x0, 0x0, 0x0,
    ];
    let mut bytes = Bytes::from(payload);
    let deserialized = StateHostInfoPayload::deserialize(&mut bytes).unwrap();
    assert_eq!(deserialized.signal, 0.539);
    assert_eq!(deserialized.tx, 55);
    assert_eq!(deserialized.rx, 55);
  }

  #[test]
  fn test_firmware_deserialize() {
    let payload: &[u8] = &[
      0xc1, 0x2b, 0x64, 0x71, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x37,
      0x0, 0x37, 0x0,
    ];
    let mut bytes = Bytes::from(payload);
    let deserialized = FirmwarePayload::deserialize(&mut bytes).unwrap();
    assert_eq!(deserialized.build, 1902390209);
    assert_eq!(deserialized.version_minor, 55);
    assert_eq!(deserialized.version_major, 55);
  }
}
