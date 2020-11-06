use crate::input_output::read_serial::IReadSerial;
use crate::serial_port::serial_port_open::SerialPortOpen;

use serde::Deserialize;
use serialport::SerialPort;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use std::{fs::File, io::prelude::*};

#[derive(Deserialize)]
struct ConfigToml {
  command: Command,
}

#[derive(Deserialize)]
struct Command {
  command_array: Vec<Vec<String>>,
}

pub struct WriteSerial<'a> {
  config_file_name: &'a str,
  read_serial: Box<dyn IReadSerial + 'a>,
  history_path: &'a str,
  max_history_len: usize,
  show_all_commands_: HashSet<String>,
}

impl<'a> WriteSerial<'a> {
  pub fn new(config_file_name: &'a str, read_serial: Box<dyn IReadSerial + 'a>) -> Self {
    let mut show_all_commands_ = HashSet::<String>::new();
    show_all_commands_.insert("SHOW ALL COMMANDS".to_uppercase());
    show_all_commands_.insert("HELP".to_uppercase());
    Self {
      config_file_name,
      read_serial,
      history_path: "history.txt",
      max_history_len: (1 << 7) + ((1 << 7) - 1),
      show_all_commands_,
    }
  }

  pub fn execute(&self, custom_command_file_name: Option<String>) {
    let custom_commands = self.get_custom_commands(custom_command_file_name);

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

    let mut rustyline_editor = Editor::<()>::new();
    rustyline_editor
      .history_mut()
      .set_max_len(self.max_history_len);

    loop {
      if count == 0 {
        println!("\n--- Press Ctrl + C to end the session ---");
        println!("--- Type in `SHOW ALL COMMANDS` for all custom commands ---");
      }
      count = (count + 1) % 5;

      let buffer_str = match self.get_input(&mut rustyline_editor) {
        Ok(line) => line,
        Err(_) => break,
      };

      let buffer_upper = buffer_str.to_uppercase();
      if self.show_all_commands_.contains(&buffer_upper) {
        self.handle_show_all_command(&custom_commands);
      } else if custom_commands.contains_key(&buffer_upper) {
        self.handle_custom_commands(&custom_commands, &buffer_upper, &mut serial_port);
      } else {
        self.write_and_read(&buffer_str, &mut serial_port);
      }
    }
  }

  pub fn get_input(&self, rustyline_editor: &mut Editor<()>) -> Result<String, ReadlineError> {
    if rustyline_editor.load_history(self.history_path).is_err() {
      println!(
        "No previous history yet. Will create history at: '{}'",
        self.history_path
      );
    }

    println!("");
    let readline = rustyline_editor.readline(">>> ");
    match readline {
      Ok(line) => {
        rustyline_editor.add_history_entry(line.as_str());
        rustyline_editor.save_history(self.history_path).unwrap();
        Ok(line)
      }
      Err(ReadlineError::Interrupted) => {
        println!("CTRL + C detected. Exiting...");
        Err(ReadlineError::Interrupted)
      }
      Err(ReadlineError::Eof) => {
        println!("CTRL +D detected");
        Err(ReadlineError::Eof)
      }
      Err(err) => {
        panic!("ERROR: {:?}", err);
      }
    }
  }

  pub fn write_and_read(&self, buffer_str: &str, serial_port: &mut Box<dyn SerialPort>) {
    println!("Tx: '{}'", buffer_str);
    self.write_str(&buffer_str, serial_port);
    self.print_read_results(serial_port);
  }

  fn write_str(&self, buffer_str: &str, serial_port: &mut Box<dyn SerialPort>) {
    let mut buffer_u8 = Vec::<u8>::new();
    for byte in buffer_str.bytes() {
      buffer_u8.push(byte);
    }
    buffer_u8.push('\n' as u8);

    let _write_result = serial_port.write(&buffer_u8);
    serial_port.flush().expect("Flush after write() failed");
    thread::sleep(Duration::from_millis(200));
  }

  fn print_read_results(&self, serial_port: &mut Box<dyn SerialPort>) {
    let lines_read = self.read_serial.read_serial_line(serial_port);
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

  fn print_buffer(&self, buf: &[u8]) {
    let mut buffer_str = String::new();
    for byte in buf {
      let deref_byte: u8 = *byte;
      buffer_str.push(deref_byte as char);
    }
    println!("Rx: '{}'", buffer_str);
  }

  fn get_custom_commands(
    &self,
    custom_command_file_name: Option<String>,
  ) -> HashMap<String, Vec<String>> {
    let mut hashmap = HashMap::<String, Vec<String>>::new();
    match custom_command_file_name {
      Some(file_name) => {
        let path: PathBuf = PathBuf::from(file_name);
        let mut file = File::open(&path).expect(&format!("Cannot open: '{}'", path.display()));
        let mut file_data = String::new();
        file.read_to_string(&mut file_data).unwrap();

        let config_toml: ConfigToml =
          toml::from_str(&file_data).expect("Cannot get values from TOML file");

        let commands_vec = config_toml.command.command_array;
        for vec_str in commands_vec {
          let mut iter = vec_str.iter();
          let shortcut_command = iter
            .next()
            .expect("Commands has no shortcut command")
            .to_uppercase();
          let commands: Vec<String> = iter.map(|str1| String::from(str1)).collect();
          hashmap.insert(String::from(shortcut_command), commands);
        }

        hashmap
      }
      None => hashmap,
    }
  }

  fn handle_show_all_command(&self, custom_commands: &HashMap<String, Vec<String>>) {
    for key in custom_commands.keys() {
      println!("Command: '{}'", key);
      let empty_vec = Vec::<String>::new();
      let iterator = custom_commands.get(key).unwrap_or(&empty_vec).iter();
      for (index, command) in iterator.enumerate() {
        println!("  {}. '{}'", index + 1, command);
      }
    }
  }

  fn handle_custom_commands(
    &self,
    custom_commands: &HashMap<String, Vec<String>>,
    buffer_upper: &str,
    serial_port: &mut Box<dyn SerialPort>,
  ) {
    let empty_vec = Vec::<String>::new();
    let vec_command = custom_commands.get(buffer_upper).unwrap_or(&empty_vec);
    for command in vec_command {
      self.write_and_read(&command, serial_port);
      let last_elem = command.split(" ").last().unwrap();
      let time_sleep_millis = match last_elem.parse::<u64>() {
        Ok(time) => time >> 1,
        Err(_) => 500,
      };
      thread::sleep(Duration::from_millis(time_sleep_millis));
    }
  }
}
