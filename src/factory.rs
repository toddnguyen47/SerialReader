use crate::input_output::read_serial::ReadSerial;
use crate::input_output::write_serial::WriteSerial;

pub struct Factory {}

impl Factory {
  pub fn create_read_serial(config_file_path: &str) -> ReadSerial {
    ReadSerial::new(config_file_path)
  }

  pub fn create_write_serial(config_file_path: &str) -> WriteSerial {
    let read_serial = Box::new(Factory::create_read_serial(config_file_path));
    WriteSerial::new(config_file_path, read_serial)
  }
}
