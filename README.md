# SerialReader

Read a serial port and display its output

# How to Build Release version

```
cargo build --release
```

# Confused?

Type in:

```
./serial-port-reader --help
```

# SerialConfig.toml

The `SerialConfig.toml` (or whatever name you named your config file) follows the settings
from the `serialport` repository: https://docs.rs/serialport/3.3.0/serialport/

# How to Read

- This program will use a file called `SerialConfig.toml` in the same directory as the executable by default.
  Ensure you have a `SerialConfig.toml` file. A sample `SerialConfig.toml` file is in the `src` folder.
- Build the release version.
- Make sure `SerialConfig.toml` is in the same directory as your executable.
- Run your executable with `./serial-port-reader read`.

# How to Write

- Same as [How to Read](#how-to-read), except use the command `./serial-port-reader write`

# How to Use - Advanced

- You can also specify a `.toml` file for the program to use by specifying the file path with the `--config` flag

```
./serial-port-reader read --config <your_toml_file_path>
./serial-port-reader write --config <your_toml_file_path>
```
