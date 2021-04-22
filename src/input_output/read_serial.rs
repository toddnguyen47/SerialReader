use std::time::Instant;

use crate::serial_port::serial_port_open::SerialPortOpen;
use chrono::{DateTime, Local};
use serialport::SerialPort;

const READ_TIMEOUT_SECONDS: u64 = 5;

pub trait IReadSerial {
    fn read_serial_line(&self, serial_port: &mut Box<dyn SerialPort>) -> Result<String, ReadError>;
}

#[derive(Debug, PartialEq)]
pub enum ReadError {
    Timeout,
    NoResponse,
}

pub struct ReadSerial<'a> {
    config_file_name: &'a str,
}

impl<'a> IReadSerial for ReadSerial<'a> {
    fn read_serial_line(&self, serial_port: &mut Box<dyn SerialPort>) -> Result<String, ReadError> {
        let mut result = String::new();
        let mut buffer: [u8; 256] = [0; 256];
        let mut is_carriage_return_char = false;
        let start_time = Instant::now();
        let mut timed_out = false;

        loop {
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

            if start_time.elapsed().as_secs() >= READ_TIMEOUT_SECONDS {
                timed_out = true;
                break;
            }

            if is_carriage_return_char {
                break;
            }
        }

        if timed_out {
            Err(ReadError::Timeout)
        } else if result.is_empty() {
            Err(ReadError::NoResponse)
        } else {
            Ok(result)
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
            match self.read_serial_line(&mut serial_port) {
                Err(_error) => {}
                Ok(line_read) => {
                    let now: DateTime<Local> = Local::now();
                    let timestamp = now.format("%Y-%m-%d %H:%M:%S");
                    let now_ms = now.timestamp_millis();
                    let delta_ms = now_ms - start_time_ms;
                    start_time_ms = now_ms;

                    let line_read = line_read.replace("\n", "\\n").replace("\r", "\\r");
                    println!("[{} {:04}ms] Rx: '{}'", timestamp, delta_ms, line_read);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use tests::spy::SerialPortSpy;

    #[test]
    fn should_give_timeout_error_when_no_response() {
        let read_serial = ReadSerial::new("foobar");
        let mut serial_port_spy: Box<dyn SerialPort> = Box::new(SerialPortSpy::new());

        let result = read_serial.read_serial_line(&mut serial_port_spy);
        assert_eq!(Err(ReadError::Timeout), result);
    }

    mod spy {
        use serialport::SerialPort;
        use std::io::{Read, Write};

        pub struct SerialPortSpy;

        impl Read for SerialPortSpy {
            fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
                return Ok(0);
            }
        }

        #[allow(unused_variables)]
        impl Write for SerialPortSpy {
            fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                todo!()
            }

            fn flush(&mut self) -> std::io::Result<()> {
                todo!()
            }
        }

        #[allow(unused_variables)]
        impl SerialPort for SerialPortSpy {
            fn name(&self) -> Option<String> {
                todo!()
            }

            fn baud_rate(&self) -> serialport::Result<u32> {
                todo!()
            }

            fn data_bits(&self) -> serialport::Result<serialport::DataBits> {
                todo!()
            }

            fn flow_control(&self) -> serialport::Result<serialport::FlowControl> {
                todo!()
            }

            fn parity(&self) -> serialport::Result<serialport::Parity> {
                todo!()
            }

            fn stop_bits(&self) -> serialport::Result<serialport::StopBits> {
                todo!()
            }

            fn timeout(&self) -> std::time::Duration {
                todo!()
            }

            fn set_baud_rate(&mut self, baud_rate: u32) -> serialport::Result<()> {
                todo!()
            }

            fn set_data_bits(&mut self, data_bits: serialport::DataBits) -> serialport::Result<()> {
                todo!()
            }

            fn set_flow_control(
                &mut self,
                flow_control: serialport::FlowControl,
            ) -> serialport::Result<()> {
                todo!()
            }

            fn set_parity(&mut self, parity: serialport::Parity) -> serialport::Result<()> {
                todo!()
            }

            fn set_stop_bits(&mut self, stop_bits: serialport::StopBits) -> serialport::Result<()> {
                todo!()
            }

            fn set_timeout(&mut self, timeout: std::time::Duration) -> serialport::Result<()> {
                todo!()
            }

            fn write_request_to_send(&mut self, level: bool) -> serialport::Result<()> {
                todo!()
            }

            fn write_data_terminal_ready(&mut self, level: bool) -> serialport::Result<()> {
                todo!()
            }

            fn read_clear_to_send(&mut self) -> serialport::Result<bool> {
                todo!()
            }

            fn read_data_set_ready(&mut self) -> serialport::Result<bool> {
                todo!()
            }

            fn read_ring_indicator(&mut self) -> serialport::Result<bool> {
                todo!()
            }

            fn read_carrier_detect(&mut self) -> serialport::Result<bool> {
                todo!()
            }

            fn bytes_to_read(&self) -> serialport::Result<u32> {
                todo!()
            }

            fn bytes_to_write(&self) -> serialport::Result<u32> {
                todo!()
            }

            fn clear(&self, buffer_to_clear: serialport::ClearBuffer) -> serialport::Result<()> {
                todo!()
            }

            fn try_clone(&self) -> serialport::Result<Box<dyn SerialPort>> {
                todo!()
            }

            fn set_break(&self) -> serialport::Result<()> {
                todo!()
            }

            fn clear_break(&self) -> serialport::Result<()> {
                todo!()
            }
        }

        impl SerialPortSpy {
            pub fn new() -> Self {
                SerialPortSpy {}
            }
        }
    }
}
