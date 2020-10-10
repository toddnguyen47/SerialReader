mod input_output;
mod parse_config;
mod serial_port;

mod factory;
use factory::Factory;

use std::thread;
use std::time::Duration;
use structopt::StructOpt;

#[derive(StructOpt)]
enum CommandsEnum {
  /// Read a serial port
  Read,
  /// Write to a serial port
  Write,
}

#[derive(StructOpt)]
#[structopt(
  name = "ReadSerial",
  about = "Reading continuously from a serial port."
)]
enum Cli {
  /// Read a serial port continuously.
  Read {
    /// Config file path.
    #[structopt(short = "-c", long = "--config")]
    config: Option<String>,
  },
  /// Write to a serial port, with custom commands
  Write {
    /// Config file path.
    #[structopt(short = "-c", long = "--config")]
    config: Option<String>,
    /// Custom command file path
    #[structopt(long = "--commands")]
    commands: Option<String>,
  },
}

fn main() {
  let args = Cli::from_args();
  match args {
    Cli::Read { config } => {
      let config_file_path: String = config.unwrap_or(String::from(""));
      let mut handlers = Vec::<thread::JoinHandle<()>>::new();
      handlers.push(thread::spawn(move || {
        let read_serial = Factory::create_read_serial(&config_file_path);
        read_serial.execute();
      }));

      handlers.push(thread::spawn(|| {
        const TIME_BETWEEN_MSG_SECONDS: u64 = 10;
        loop {
          println!("Press Ctrl + C to end the session");
          thread::sleep(Duration::from_secs(TIME_BETWEEN_MSG_SECONDS));
        }
      }));

      for handler in handlers {
        handler.join().expect("Handler did not join()");
      }
    }

    Cli::Write { config, commands } => {
      let config_file_path: String = config.unwrap_or(String::from(""));
      let write_serial = Factory::create_write_serial(&config_file_path);

      write_serial.execute(commands);
    }
  }
}
