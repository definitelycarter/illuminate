use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt;

#[repr(u16)]
#[derive(PartialEq, Debug, Copy, Clone, TryFromPrimitive, IntoPrimitive)]
pub enum MessageType {
  GetService = 2,
  StateService = 3,
  GetHostInfo = 12,
  StateHostInfo = 13,
  GetHostFirmware = 14,
  StateHostFirmware = 15,
  GetWifiInfo = 16,
  StateWifiInfo = 17,
  GetWifiFirmware = 18,
  StateWifiFirmware = 19,
  GetLabel = 23,
  SetLabel = 24,
  StateLabel = 25,
  GetVersion = 32,
  StateVersion = 33,
  GetInfo = 34,
  StateInfo = 35,
  Acknowlegement = 45,
  GetLocation = 48,
  SetLocation = 49,
  StateLocation = 50,
  GetGroup = 51,
  SetGroup = 52,
  StateGroup = 53,

  Get = 101,
  SetColor = 102,
  State = 107,

  GetPower = 116,
  SetPower = 117,
  StatePower = 118,
}

#[repr(u16)]
#[derive(Debug, Copy, Clone, TryFromPrimitive, IntoPrimitive)]
pub enum Power {
  On = 65535,
  Off = 0,
}

impl fmt::Display for MessageType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    fmt::Debug::fmt(self, f)
  }
}

impl fmt::Display for Power {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    fmt::Debug::fmt(self, f)
  }
}
