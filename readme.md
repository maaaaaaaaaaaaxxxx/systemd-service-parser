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
