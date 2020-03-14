use dotenv::dotenv;
use lifx::{Client, MessageType, StatePayload};
use std::convert::TryInto;
use std::net::UdpSocket;
use std::sync::Arc;
use storage::Storage;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenv().ok().unwrap_or_default();
  let addr = "0.0.0.0:0".to_string();

  let db_url = std::env::var("DATABASE_URL")?;

  let udp_socket = UdpSocket::bind(&addr)?;
  let client = Arc::new(Client::new(1337, udp_socket)?);

  let discoverer = Arc::clone(&client);
  tokio::spawn(async move {
    loop {
      discoverer.get_state().await.unwrap();
      tokio::time::delay_for(std::time::Duration::from_secs(5)).await;
    }
  });

  let receiver = Arc::clone(&client);
  let storage = Arc::new(tokio::sync::Mutex::new(Storage::new(&db_url)));
  let task = tokio::spawn(async move {
    loop {
      let (_, packet) = receiver.receive_message().await.unwrap();
      match packet.message_type() {
        MessageType::State => {
          let storage = &*storage.lock().await;
          let state_firmware: StatePayload = packet.try_into().unwrap();
          let label = String::from_utf8(state_firmware.label.to_vec()).unwrap();
          if let Some(existing) = storage.get_device_by_label(&label) {
            storage.update_device(&existing).unwrap();
          } else {
            storage.insert_device(storage::NewDevice { label }).unwrap();
          }
        }
        _ => continue,
      }
    }
  });

  tokio::join!(task);
  Ok(())
}
