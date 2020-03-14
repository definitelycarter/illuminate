use super::models;
use super::schema;
use diesel::prelude::*;
use diesel::SqliteConnection;
use uuid::Uuid;

embed_migrations!("./migrations");

pub struct Storage {
  inner: SqliteConnection,
}

impl Storage {
  pub fn new(database_url: &str) -> Self {
    let connection = SqliteConnection::establish(database_url)
      .unwrap_or_else(|_| panic!("Database does not exist"));

    embedded_migrations::run_with_output(&connection, &mut std::io::stdout()).unwrap();

    Storage { inner: connection }
  }

  pub fn get_devices(&self) -> anyhow::Result<Vec<models::Device>> {
    use schema::devices::dsl::*;
    devices
      .load::<models::Device>(&self.inner)
      .map_err(|_| anyhow::Error::msg("Unable to get devices"))
  }

  pub fn insert_device(&self, device: models::NewDevice) -> anyhow::Result<models::Device> {
    use schema::devices::dsl::*;

    let inserted = models::Device {
      id: Uuid::new_v4().to_hyphenated().to_string(),
      label: device.label,
      power: None,
      hue: None,
      kelvin: None,
      saturation: None,
      brightness: None,
    };

    diesel::insert_into(devices)
      .values(&inserted)
      .execute(&self.inner)
      .map_err(|err| {
        println!("{}", err);
        anyhow::Error::msg("Unable to insert device")
      })?;

    Ok(inserted)
  }

  pub fn update_device(&self, device: &models::Device) -> anyhow::Result<usize> {
    use schema::devices::dsl::*;
    diesel::update(devices.filter(id.eq(&device.id)))
      .set(device)
      .execute(&self.inner)
      .map_err(|err| {
        println!("{}", err);
        anyhow::Error::msg("Unable to update device")
      })
  }

  pub fn delete_device(&self, device_id: &String) -> anyhow::Result<usize> {
    use schema::devices::dsl::*;
    diesel::delete(devices.filter(id.eq(device_id)))
      .execute(&self.inner)
      .map_err(|_| anyhow::Error::msg("Unable to update device"))
  }

  pub fn get_device_by_label(&self, device_label: &String) -> Option<models::Device> {
    use schema::devices::dsl::*;
    devices
      .filter(label.eq(device_label))
      .first(&self.inner)
      .optional()
      .map_err(|err| {
        println!("{}", err);
        anyhow::Error::msg("Unable to query device")
      })
      .unwrap()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    let storage = Storage::new("");
    let device = models::NewDevice {
      label: "Adam's".to_string(),
    };
    storage.insert_device(device).unwrap();
    let devices = storage.get_devices().unwrap();
    assert_eq!(devices.len(), 1);

    let device = &devices[0];
    storage.delete_device(&device.id).unwrap();

    let devices = storage.get_devices().unwrap();
    assert_eq!(devices.len(), 0);
  }
}
