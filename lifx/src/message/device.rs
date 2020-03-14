pub struct StateServicePayload {
  pub service: u8,
  pub port: u32,
}

pub struct StateHostInfoPayload {
  pub signal: f32,
  pub tx: u32,
  pub rx: u32,
}

pub struct FirmwarePayload {
  pub build: u64,
  pub version_minor: u16,
  pub version_major: u16,
}

pub struct StateWifiInfoPayload {
  pub signal: f32,
  pub tx: u32,
  pub rx: u32,
}

pub struct StateVersionPayload {
  pub vendor: u32,
  pub product: u32,
  pub version: u32,
}

pub struct StateInfoPayload {
  pub time: u64,
  pub uptime: u64,
  pub downtime: u64,
}

pub struct LocationPayload {
  pub location: [u8; 16],
  pub label: [u8; 32],
  pub updated_at: u64,
}

pub struct GroupPayload {
  pub group: [u8; 16],
  pub label: [u8; 32],
  pub updated_at: u64, // docs say i64??
}

pub struct EchoPayload {
  pub payload: [u8; 64],
}
