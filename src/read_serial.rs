use chrono::{DateTime, Local};
use serialport;

use crate::parse_config::{GetConfigResults, ParseConfig};
pub struct ReadSerial;

impl ReadSerial {
    pub fn new() -> ReadSerial {
        ReadSerial {}
    }

    pub fn execute(&self, config_file_name: &str) {
        let toml_config_result: GetConfigResults = ParseConfig::get_config(config_file_name);
        let port = toml_config_result.serial_port;
        let settings = toml_config_result.serial_port_settings;
        let mut serial_port = serialport::open_with_settings(&port, &settings)
            .expect(&format!("Serial Port did not open!\n'{}'", port));

        let mut buffer = [0; 256];
        let mut string_result = String::new();
        loop {
            let bytes_read = serial_port.read(&mut buffer).unwrap_or(0);
            if bytes_read > 0 {
                let mut is_carriage_return_char = false;

                for i in 0..bytes_read {
                    let c1 = buffer[i] as char;
                    if c1 == '\r' || c1 == '\n' {
                        is_carriage_return_char = true;
                    }
                    string_result.push(buffer[i] as char);
                }

                if is_carriage_return_char {
                    let now: DateTime<Local> = Local::now();
                    let timestamp = now.format("%Y-%m-%d %H:%M:%S");
                    string_result = string_result.replace("\n", "\\n").replace("\r", "\\r");
                    println!("[{}] Rx: '{}'", timestamp, string_result);
                    string_result.clear();
                }
            }
        }
    }
}
