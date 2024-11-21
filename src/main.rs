use std::{env, fs};
use systemd_service_parser::parse_service_file;
use pest::Parser;
use serde_json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    // Taking file for input
    if args.len() != 2 {
        eprintln!("Usage: {} <path_to_service_file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    // Parsing
    let service_file = fs::read_to_string(file_path)?;

    let service = systemd_service_parser::SystemdServiceParser::parse(systemd_service_parser::Rule::file, &service_file);
    println!("{:#?}", service);

    // Converting to JSON
    match parse_service_file(&service_file) {
        Ok(service_data) => {
            let json_output = serde_json::to_string_pretty(&service_data)?;
            println!("{}", json_output);
        }
        Err(e) => {
            eprintln!("Error parsing service file: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
