use super::proto::OutgoingPacket;
use std::convert::TryInto;
use std::net::SocketAddr;
use tokio::net::udp::SendHalf;

pub struct Writer {
  send_half: SendHalf,
}

impl Writer {
  pub fn new(send_half: SendHalf) -> Self {
    Self { send_half }
  }

  pub async fn write_packet(
    &mut self,
    addr: &SocketAddr,
    packet: OutgoingPacket,
  ) -> anyhow::Result<()> {
    let bytes: Vec<u8> = packet.try_into()?;
    self.send_half.send_to(&bytes, &addr).await?;
    Ok(())
  }
}
