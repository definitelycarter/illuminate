use bytes::{Bytes, BytesMut};

pub trait Serializable {
  fn serialize(&self, bytes: &mut BytesMut) -> anyhow::Result<()>;
}

pub trait Deserializable {
  fn deserialize(bytes: &mut Bytes) -> anyhow::Result<Self>
  where
    Self: Sized;
}
