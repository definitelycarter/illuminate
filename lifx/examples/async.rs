use lifx::*;
use log::{info, trace};
use std::convert::TryInto;
use std::net::UdpSocket;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  simple_logger::init_with_level(log::Level::Info)?;
  let addr = "0.0.0.0:0".to_string();
  let socket = UdpSocket::bind(&addr)?;
  let client = Arc::new(Client::new(1337, socket)?);

  let sender = Arc::clone(&client);
  tokio::spawn(async move {
    loop {
      sender.get_state().await.unwrap();
      tokio::time::delay_for(Duration::from_secs(5)).await;
    }
  });

  loop {
    let timer = timeout(Duration::from_secs(1), client.receive_message()).await;

    if let Err(_) = timer {
      info!("did not receive message");
      continue;
    }

    let (addr, packet) = timer?.unwrap();

    trace!(
      "addr {}, target {}, {:x?}",
      addr,
      packet.target(),
      packet.target().to_be_bytes()
    );

    match packet.message_type() {
      MessageType::State => handle_state(packet.try_into()?),
      _ => continue,
    }
  }
}

pub fn handle_state(payload: StatePayload) {
  let label = std::str::from_utf8(&payload.label)
    .unwrap()
    .trim_end_matches(char::from(0));
  info!("power: {}, label: {}", payload.power(), label)
}
