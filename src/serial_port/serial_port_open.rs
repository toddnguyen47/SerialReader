use crate::parse_config::{GetConfigResults, ParseConfig};
use serialport::SerialPort;
use std::time::Duration;

/// Struct returned by `get_serial_port()`
pub struct SerialPortResults {
  /// Serial port object used for reading and writing
  pub serial_port: Box<dyn SerialPort>,
  /// Timeout Duration object
  pub timeout_duration: Duration,
}
pub struct SerialPortOpen {}

impl SerialPortOpen {
  pub fn get_serial_port(config_file_name: &str) -> SerialPortResults {
    let toml_config_result: GetConfigResults = ParseConfig::get_config(config_file_name);
    let port = toml_config_result.serial_port;
    let settings = toml_config_result.serial_port_settings;
    let serial_port = serialport::open_with_settings(&port, &settings).expect(&format!(
      "\nSerial Port did not open!\nSerial Port: `{}`\n",
      port
    ));
    println!("Opening serial port: '{}'", port);
    SerialPortResults {
      serial_port,
      timeout_duration: settings.timeout,
    }
  }
}
