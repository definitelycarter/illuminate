use crate::message::*;
use crate::proto::{IncomingPacket, MessageType, OutgoingPacket, Power};
use crate::reader::Reader;
use crate::writer::Writer;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;

const EMPTY_PAYLOAD: EmptyPayload = EmptyPayload {};

pub struct Client {
  id: u32,
  reader: Arc<Mutex<Reader>>,
  writer: Arc<Mutex<Writer>>,
}

impl Client {
  pub fn new(id: u32, socket: std::net::UdpSocket) -> anyhow::Result<Self> {
    let socket = UdpSocket::from_std(socket)?;
    socket.set_broadcast(true)?;
    let (recv_half, send_half) = socket.split();

    let reader = Reader::new(recv_half);
    let writer = Writer::new(send_half);

    Ok(Self {
      id,
      reader: Arc::new(Mutex::new(reader)),
      writer: Arc::new(Mutex::new(writer)),
    })
  }

  pub async fn get_service(&self) -> anyhow::Result<()> {
    let packet = OutgoingPacket::new(
      0,
      self.id,
      false,
      true,
      MessageType::GetService,
      EMPTY_PAYLOAD,
    )?;
    self.send_packet(packet).await
  }

  pub async fn get_host_info(&self) -> anyhow::Result<()> {
    let packet = OutgoingPacket::new(
      0,
      self.id,
      false,
      true,
      MessageType::GetHostInfo,
      EMPTY_PAYLOAD,
    )?;
    self.send_packet(packet).await
  }

  pub async fn get_host_firmware(&self) -> anyhow::Result<()> {
    let packet = OutgoingPacket::new(
      0,
      self.id,
      false,
      true,
      MessageType::GetHostFirmware,
      EMPTY_PAYLOAD,
    )?;
    self.send_packet(packet).await
  }

  pub async fn get_wifi_info(&self) -> anyhow::Result<()> {
    let packet = OutgoingPacket::new(
      0,
      self.id,
      false,
      true,
      MessageType::GetWifiFirmware,
      EMPTY_PAYLOAD,
    )?;
    self.send_packet(packet).await
  }

  pub async fn set_power(&self, level: Power, duration: u32) -> anyhow::Result<()> {
    let packet = OutgoingPacket::new(
      0,
      self.id,
      true,
      false,
      MessageType::SetPower,
      SetPowerPayload { level, duration },
    )?;
    self.send_packet(packet).await
  }

  pub async fn set_color(&self, color: Color, duration: u32) -> anyhow::Result<()> {
    let payload = SetColorPayload { color, duration };
    let packet = OutgoingPacket::new(0, self.id, true, false, MessageType::SetColor, payload)?;
    self.send_packet(packet).await
  }

  pub async fn get_state(&self) -> anyhow::Result<()> {
    let packet = OutgoingPacket::new(0, self.id, false, true, MessageType::Get, EMPTY_PAYLOAD)?;
    self.send_packet(packet).await
  }

  pub async fn send_packet(&self, packet: OutgoingPacket) -> anyhow::Result<()> {
    let mut writer = self.writer.lock().await;
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(255, 255, 255, 255)), 56700);
    writer.write_packet(&addr, packet).await
  }

  pub async fn receive_message(&self) -> anyhow::Result<(SocketAddr, IncomingPacket)> {
    let mut reader = self.reader.lock().await;
    reader.read_packet().await
  }
}
