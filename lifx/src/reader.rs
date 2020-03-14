use super::proto::{Deserializable, IncomingPacket};
use bytes::Bytes;
use log::trace;
use std::net::SocketAddr;
use tokio::net::udp::RecvHalf;

pub struct Reader {
  recv_half: RecvHalf,
}

impl Reader {
  pub fn new(recv_half: RecvHalf) -> Self {
    Self { recv_half }
  }

  pub async fn read_packet(&mut self) -> anyhow::Result<(SocketAddr, IncomingPacket)> {
    let mut buf = [0; 256];
    let (amt, addr) = self.recv_half.recv_from(&mut buf).await?;
    let read = &buf[0..amt];
    trace!("addr: {}, read {:x?}", addr, read);
    let mut bytes = Bytes::copy_from_slice(read);
    let packet = IncomingPacket::deserialize(&mut bytes)?;
    Ok((addr, packet))
  }
}
