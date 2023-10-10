# Tcpdrop

Export IPv4 and IPv6 TCP table.

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
