# Tcpdrop

Export IPv4 and IPv6 TCP table.

## Installation

### Cargo

Make sure the current stable release of [Rust](https://rust-lang.org/tools/install) is installed.

#### Registry

```bash
cargo install tcpdrop
```

#### Manual

```bash
git clone https://github.com/ynuwenhof/tcpdrop.git
cd tcpdrop
cargo install --path .
```

After installing, you can run the application with:

```bash
tcpdrop --pretty
```

this will print the TCP table into the terminal.


## Configuration

Tcpdrop can be configured via environment variables or command line arguments.

Missing keys will fall back to their default value.

| Key              | Description                                 | Default |
|------------------|---------------------------------------------|---------|
| `TCPDROP_PRETTY` | Print the TCP table in a pretty JSON format | `false` |
| `TCPDROP_NO_V4`  | Don't export IPv4 connections               | `false` |
| `TCPDROP_NO_V6`  | Don't export IPv6 connections               | `false` |
| `TCPDROP_OUTPUT` | Output file path                            | `None`  |

## License

This project is licensed under either of the following licenses, at your option:

* [Apache License, Version 2.0](https://apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](https://github.com/ynuwenhof/tcpdrop/blob/main/LICENSE-APACHE))
* [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](https://github.com/ynuwenhof/tcpdrop/blob/main/LICENSE-MIT))
