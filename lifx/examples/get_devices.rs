use lifx::*;
use log::info;
use std::convert::TryInto;
use std::net::UdpSocket;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  simple_logger::init_with_level(log::Level::Info)?;
  let addr = "0.0.0.0:0".to_string();
  let socket = UdpSocket::bind(&addr)?;
  let client = Client::new(1337, socket)?;

  // client.set_power(Power::On, 11000).await?;
  client.get_state().await?;

  loop {
    let (addr, packet) = client.receive_message().await?;
    match packet.message_type() {
      MessageType::State => handle_state(packet.try_into()?),
      _ => continue,
    }
  }
}

pub fn handle_state(payload: StatePayload) {
  let label = String::from_utf8(payload.label.to_vec()).unwrap();
  info!(
    "label: {}, power: {}, color: {}",
    label,
    payload.power(),
    payload.color
  );
}
