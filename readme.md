# systemd-service-parser
A simple systemd service file parser written on Rust using `pest` library which extracts sections and their key-value pairs.

## Grammar definition
WHITESPACE = _{ " " | "\t" } - ignores whitespaces

file = { SOI ~ (SECTION | COMMENT | NEWLINE)+ ~ EOI } - The file can consist of section definitions, comments and newlines
COMMENT = { "#" ~ (!NEWLINE ~ ANY)* ~ NEWLINE }
SECTION = { HEADER ~ NEWLINE+ ~ KEY_VALUE* ~ NEWLINE* }
HEADER = { "[" ~ ASCII_ALPHANUMERIC+ ~ "]" } - Section definition
KEY_VALUE = { KEY ~ "=" ~ VALUE ~ NEWLINE } - Key-value pairs
KEY = { (ASCII_ALPHANUMERIC | "-" | "_")+ }
VALUE = { (!NEWLINE ~ ANY)* }
NEWLINE = _{ "\r\n" | "\n" }

## Example systemd service file
```service
[Unit]
Description=A systemd service.

[Service]
ExecStart=/usr/bin/service

[Install]
WantedBy=multi-user.target
```

## Example usage
```
cargo run -- test.service
```
**Output:**
```json
{
  "sections": [
    {
      "header": "Unit",
      "key_values": [
        {
          "key": "Description",
          "value": "A systemd service."
        }
      ]
    },
    {
      "header": "Service",
      "key_values": [
        {
          "key": "ExecStart",
          "value": "/usr/bin/service"
        }
      ]
    },
    {
      "header": "Install",
      "key_values": [
        {
          "key": "WantedBy",
          "value": "multi-user.target"
        }
      ]
    }
  ]
}
```
## Links
**crates.io** - https://crates.io/crates/systemd_service_parser
**docs.rs** - https://docs.rs/systemd_service_parser/0.1.0/systemd_service_parser
