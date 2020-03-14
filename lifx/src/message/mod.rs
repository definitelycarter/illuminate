mod color;
mod device;
mod light;

pub use color::*;
pub use device::*;
pub use light::*;

use crate::proto::Serializable;

pub struct EmptyPayload {}

impl Serializable for EmptyPayload {
  fn serialize(&self, _: &mut bytes::BytesMut) -> anyhow::Result<()> {
    Ok(())
  }
}
