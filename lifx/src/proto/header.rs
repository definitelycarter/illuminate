use super::message::MessageType;
use super::serialize::{Deserializable, Serializable};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::convert::TryFrom;

const ADDRESSABLE: u16 = 0b0001_0000_0000_0000;
const PROTOCOL: u16 = 0b0000_0100_0000_0000;
const TAGGED: u16 = 0b0010_0000_0000_0000;

const RESPONSE_REQUIRED: u8 = 0b0000_0001;
const ACKNOWLEGEMENT_REQUIRED: u8 = 0b0000_0010;

pub(crate) struct Header {
  pub size: u16,
  pub proto: u16,
  pub addressable: bool,
  pub tagged: bool,
  pub source: u32,

  pub target: u64,
  pub res_required: bool,
  pub ack_required: bool,
  pub sequence: u8,

  pub message_type: MessageType,
}

impl Header {
  pub fn new(
    size: u16,
    tagged: bool,
    source: u32,
    target: u64,
    ack_required: bool,
    res_required: bool,
    sequence: u8,
    message_type: MessageType,
  ) -> Header {
    Header {
      size,
      proto: PROTOCOL,
      addressable: true,
      tagged,
      source,
      target,
      ack_required,
      res_required,
      sequence,
      message_type,
    }
  }
}

impl Serializable for Header {
  fn serialize(&self, bytes: &mut BytesMut) -> anyhow::Result<()> {
    // --- Frame
    bytes.put_u16_le(self.size);

    let mut flags = self.proto; // protocol is always 1024
    if self.tagged {
      flags |= TAGGED;
    }
    if self.addressable {
      flags |= ADDRESSABLE;
    }
    bytes.put_u16_le(flags);

    bytes.put_u32_le(self.source);

    // -- Frame Address

    bytes.put_u64_le(self.target);

    // reserve 6 bytes
    for _ in 0..6 {
      bytes.put_u8(0);
    }

    let mut flags = 0_u8;
    if self.res_required {
      flags |= RESPONSE_REQUIRED;
    }
    if self.ack_required {
      flags |= ACKNOWLEGEMENT_REQUIRED;
    }
    bytes.put_u8(flags);

    bytes.put_u8(self.sequence);

    // -- Protocol Header

    // reserve 8 bytes
    for _ in 0..8 {
      bytes.put_u8(0);
    }
    bytes.put_u16_le(self.message_type.into());
    // reserve 2 bytes
    for _ in 0..2 {
      bytes.put_u8(0);
    }
    Ok(())
  }
}

impl Deserializable for Header {
  fn deserialize(bytes: &mut Bytes) -> anyhow::Result<Self> {
    // --- Frame

    let size = bytes.get_u16_le();

    let flags = bytes.get_u16_le();
    let tagged = (flags & TAGGED) > 0;
    let addressable = (flags & ADDRESSABLE) > 0;

    let source = bytes.get_u32_le();

    // -- Frame Address

    let target = bytes.get_u64_le();
    // skip 6 bytes
    bytes.advance(6);

    let flags = bytes.get_u8();
    let res_required = (flags & RESPONSE_REQUIRED) > 0;
    let ack_required = (flags & ACKNOWLEGEMENT_REQUIRED) > 0;

    let sequence = bytes.get_u8();

    // --- Protocol Header

    // skip 8 bytes
    bytes.advance(8);
    let message_type = bytes.get_u16_le();
    let message_type = MessageType::try_from(message_type)?;
    // skip 2 bytes
    bytes.advance(2);

    Ok(Header {
      size,
      proto: PROTOCOL,
      addressable,
      tagged,
      source,

      target,
      res_required,
      ack_required,
      sequence,

      message_type,
    })
  }
}
