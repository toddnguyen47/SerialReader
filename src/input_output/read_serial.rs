use crate::serial_port::serial_port_open::SerialPortOpen;
use chrono::{DateTime, Local};
use serialport::SerialPort;

pub trait IReadSerial {
  fn read_serial_line(&self, serial_port: &mut Box<dyn SerialPort>) -> Option<String>;
}

pub struct ReadSerial<'a> {
  config_file_name: &'a str,
}

impl<'a> IReadSerial for ReadSerial<'a> {
  fn read_serial_line(&self, serial_port: &mut Box<dyn SerialPort>) -> Option<String> {
    let mut result = String::new();
    let mut buffer: [u8; 256] = [0; 256];
    let mut is_carriage_return_char = false;

    while false == is_carriage_return_char {
      let bytes_read = serial_port.read(&mut buffer).unwrap_or(0);
      if bytes_read > 0 {
        for i in 0..bytes_read {
          let c1 = buffer[i] as char;
          if c1 == '\r' || c1 == '\n' {
            is_carriage_return_char = true;
          }
          result.push(buffer[i] as char);
        }
      }
    }

    if result.is_empty() {
      None
    } else {
      Some(result)
    }
  }
}

impl<'a> ReadSerial<'a> {
  pub fn new(config_file_name: &'a str) -> ReadSerial {
    ReadSerial { config_file_name }
  }

  pub fn execute(&self) {
    let serial_port_results = SerialPortOpen::get_serial_port(self.config_file_name);
    let mut serial_port = serial_port_results.serial_port;
    let mut start_time_ms = Local::now().timestamp_millis();

    loop {
      let mut string_result = self.read_serial_line(&mut serial_port).unwrap();
      let now: DateTime<Local> = Local::now();
      let timestamp = now.format("%Y-%m-%d %H:%M:%S");
      let now_ms = now.timestamp_millis();
      let delta_ms = now_ms - start_time_ms;
      start_time_ms = now_ms;
      string_result = string_result.replace("\n", "\\n").replace("\r", "\\r");
      println!("[{} {:04}ms] Rx: '{}'", timestamp, delta_ms, string_result);
    }
  }
}
