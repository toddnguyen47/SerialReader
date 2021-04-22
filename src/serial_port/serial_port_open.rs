use crate::parse_config::ParseConfig;
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
        let parsed_toml_values = ParseConfig::get_config(config_file_name);
        let port = &parsed_toml_values.serial_port;
        let timeout_duration = parsed_toml_values.timeout_in_milliseconds;

        let serial_port = serialport::new(port.to_string(), parsed_toml_values.baud_rate);
        let serial_port = serial_port.data_bits(parsed_toml_values.data_bits);
        let serial_port = serial_port.flow_control(parsed_toml_values.flow_control);
        let serial_port = serial_port.parity(parsed_toml_values.parity);
        let serial_port = serial_port.stop_bits(parsed_toml_values.stop_bits);
        let serial_port = serial_port.timeout(timeout_duration);

        let serial_port = serial_port.open().expect(&format!(
            "\nSerial Port did not open!\nSerial Port: `{}`\n",
            &port
        ));
        println!("Opening serial port: '{}'", &port);

        SerialPortResults {
            serial_port,
            timeout_duration,
        }
    }
}
