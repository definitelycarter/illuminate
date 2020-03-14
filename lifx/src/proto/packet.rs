use super::message::MessageType;
use super::serialize::{Deserializable, Serializable};
use super::Header;
use bytes::{Buf, Bytes, BytesMut};

use std::convert::TryInto;

pub struct OutgoingPacket {
  header: Header,
  payload: bytes::Bytes,
}

impl OutgoingPacket {
  pub fn new(
    sequence: u8,
    source: u32,
    ack_required: bool,
    res_required: bool,
    message_type: MessageType,
    payload: impl Serializable,
  ) -> anyhow::Result<Self> {
    let mut bytes = BytesMut::new();
    payload.serialize(&mut bytes)?;
    let len = bytes.len() as u16;

    let header = Header::new(
      36_u16 + len,
      true,
      source,
      0,
      ack_required,
      res_required,
      sequence,
      message_type,
    );
    Ok(OutgoingPacket {
      header,
      payload: bytes.to_bytes(),
    })
  }
}

impl TryInto<Vec<u8>> for OutgoingPacket {
  type Error = anyhow::Error;
  fn try_into(self) -> Result<Vec<u8>, Self::Error> {
    let mut bytes = BytesMut::new();
    self.header.serialize(&mut bytes)?;
    let mut vec = bytes.to_vec();
    vec.extend(self.payload);
    Ok(vec)
  }
}

pub struct IncomingPacket {
  header: Header,
  payload: Bytes,
}

impl IncomingPacket {
  pub fn payload(&self) -> Bytes {
    self.payload.to_owned()
  }

  pub fn message_type(&self) -> MessageType {
    self.header.message_type
  }

  pub fn target(&self) -> u64 {
    self.header.target
  }
}

impl TryInto<crate::message::StatePayload> for IncomingPacket {
  type Error = anyhow::Error;
  fn try_into(mut self) -> Result<crate::message::StatePayload, Self::Error> {
    crate::message::StatePayload::deserialize(&mut self.payload)
  }
}

impl Deserializable for IncomingPacket {
  fn deserialize(bytes: &mut Bytes) -> anyhow::Result<Self> {
    let header = Header::deserialize(bytes)?;
    let payload = bytes.to_bytes();
    Ok(Self { header, payload })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn should_decode_header() {
    let payload = vec![
      0x31, 0x00, 0x00, 0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x66, 0x00, 0x00, 0x00, 0x00, 0x55, 0x55, 0xFF, 0xFF, 0xFF, 0xFF, 0xAC, 0x0D,
      0x00, 0x04, 0x00, 0x00,
    ];

    let mut bytes = Bytes::from(payload);
    let packet = IncomingPacket::deserialize(&mut bytes).unwrap();

    // let packet = Packet::from_bytes(&payload).unwrap();
    assert_eq!(packet.message_type(), MessageType::SetColor);

    println!("{:#x?}", packet.payload().to_vec());
  }
}
