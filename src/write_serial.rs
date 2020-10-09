use crate::{read_serial::IReadSerial, serial_port_open::SerialPortOpen};

use std::io;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

pub struct WriteSerial<'a> {
  config_file_name: &'a str,
  read_serial: Box<dyn IReadSerial + 'a>,
}

impl<'a> WriteSerial<'a> {
  pub fn new(config_file_name: &'a str, read_serial: Box<dyn IReadSerial + 'a>) -> Self {
    Self {
      config_file_name,
      read_serial,
    }
  }

  pub fn execute(&self) {
    let mut buffer_arr: [u8; 256] = [0; 256];
    let serial_port_results = SerialPortOpen::get_serial_port(self.config_file_name);
    let mut serial_port = serial_port_results.serial_port;

    // Initial flush
    while serial_port.read(&mut buffer_arr).is_ok() {
      serial_port.flush().expect("Initial flush failed");
      self.print_buffer(&buffer_arr);
    }

    let _timeout_duration = serial_port_results.timeout_duration;
    let mut count = 0;
    let stdin = io::stdin();

    loop {
      if count == 0 {
        println!("Press Ctrl + C to end the session");
      }
      count = (count + 1) % 10;

      print!("\n>>> ");
      io::stdout().flush().expect("Failed to flush prompt");
      let mut buffer_str = String::new();
      stdin
        .read_line(&mut buffer_str)
        .expect("Failed to read line");
      buffer_str.pop();
      println!("Tx: '{}'", buffer_str);

      let mut buffer_u8 = Vec::<u8>::new();
      for byte in buffer_str.bytes() {
        buffer_u8.push(byte);
      }
      buffer_u8.push('\n' as u8);

      let _ = match serial_port.write(&buffer_u8) {
        Ok(bytes) => bytes,
        Err(_) => 0,
      };
      serial_port.flush().expect("Flush after write() failed");
      thread::sleep(Duration::from_millis(200));

      let lines_read = self.read_serial.read_serial_line(&mut serial_port);
      match lines_read {
        Some(line) => {
          let replaced_line = line.replace("\r", "\\r");
          for cur_line in replaced_line.split("\n") {
            if !cur_line.is_empty() {
              println!("Rx: '{}'", cur_line)
            }
          }
        }
        None => {}
      }
    }
  }

  fn print_buffer(&self, buf: &[u8]) {
    let mut buffer_str = String::new();
    for byte in buf {
      let deref_byte: u8 = *byte;
      buffer_str.push(deref_byte as char);
    }
    println!("Rx: '{}'", buffer_str);
  }
}
