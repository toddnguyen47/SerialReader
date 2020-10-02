mod parse_config;

mod read_serial;
use read_serial::ReadSerial;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "ReadSerial",
    about = "Reading continuously from a serial port."
)]
struct Cli {
    /// `toml` File Path
    #[structopt(long = "--config")]
    config: Option<String>,
}

fn main() {
    let args = Cli::from_args();
    let config_file_path: String = match args.config {
        Some(string_val) => string_val,
        None => String::from(""),
    };

    let read_serial = ReadSerial::new();
    read_serial.execute(&config_file_path);
}
