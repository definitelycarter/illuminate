use crate::schema::*;

#[derive(AsChangeset, Insertable, Queryable)]
pub struct Device {
  pub id: String,
  pub label: String,
  pub power: Option<i32>,
  pub hue: Option<i32>,
  pub saturation: Option<i32>,
  pub brightness: Option<i32>,
  pub kelvin: Option<i32>,
}

pub struct NewDevice {
  pub label: String,
}
