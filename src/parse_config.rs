use serde::Deserialize;
use serialport::{DataBits, FlowControl, Parity, SerialPortSettings, StopBits};
use std::env;
use std::fs::File;
use std::io::Read;
use std::time::Duration;

#[derive(Deserialize)]
struct ConfigToml {
    serial: Serial,
}

/**
 * The name of the struct has to match the name of the section,
 * e.g. [serial] will have to have a struct named Serial
 */
#[derive(Deserialize)]
struct Serial {
    baud_rate: u32,
    data_bits: u32,
    flow_control: String,
    parity: String,
    stop_bits: u32,
    timeout_in_milliseconds: u64,
}

pub struct ParseConfig;

impl ParseConfig {
    pub fn get_config() -> SerialPortSettings {
        let mut path = env::current_exe().unwrap();
        let config_file_name = "SerialConfig.toml";
        path.pop();
        path.push(config_file_name);

        let mut file = File::open(path).expect("Cannot open SerialConfig.toml");
        let mut file_data = String::new();
        file.read_to_string(&mut file_data).unwrap();

        let config_toml: ConfigToml =
            toml::from_str(&file_data).expect("Cannot get values from TOML file");
        let toml_val = config_toml.serial;
        let baud_rate = toml_val.baud_rate;
        let data_bits = ParseConfig::get_data_bits(&toml_val);
        let flow_control = ParseConfig::get_flow_control(&toml_val);
        let parity = ParseConfig::get_parity(&toml_val);
        let stop_bits = ParseConfig::get_stop_bits(&toml_val);
        let timeout = Duration::from_millis(toml_val.timeout_in_milliseconds);
        SerialPortSettings {
            baud_rate,
            data_bits,
            flow_control,
            parity,
            stop_bits,
            timeout,
        }
    }

    fn get_data_bits(toml_val: &Serial) -> DataBits {
        match toml_val.data_bits {
            5 => DataBits::Five,
            6 => DataBits::Six,
            7 => DataBits::Seven,
            8 => DataBits::Eight,
            _ => DataBits::Eight,
        }
    }

    fn get_flow_control(toml_val: &Serial) -> FlowControl {
        let flow_control: &str = &toml_val.flow_control.to_lowercase();
        match flow_control {
            "none" => FlowControl::None,
            "software" => FlowControl::Software,
            "hardware" => FlowControl::Hardware,
            _ => FlowControl::None,
        }
    }

    fn get_parity(toml_val: &Serial) -> Parity {
        let parity: &str = &toml_val.parity.to_lowercase();
        match parity {
            "none" => Parity::None,
            "odd" => Parity::Odd,
            "even" => Parity::Even,
            _ => Parity::None,
        }
    }

    fn get_stop_bits(toml_val: &Serial) -> StopBits {
        match toml_val.stop_bits {
            1 => StopBits::One,
            2 => StopBits::Two,
            _ => StopBits::One,
        }
    }
}
