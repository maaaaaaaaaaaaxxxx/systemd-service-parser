use pest::Parser;
use pest_derive::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct SystemdServiceParser;

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValuePair {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Section {
    pub header: String,
    pub key_values: Vec<KeyValuePair>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemdService {
    pub sections: Vec<Section>,
}

pub fn parse_service_file(content: &str) -> Result<SystemdService, Box<dyn std::error::Error>> {
    let parsed = SystemdServiceParser::parse(Rule::file, content)?;

    let mut sections = Vec::new();

    for record in parsed {
        match record.as_rule() {
            Rule::file => {
                for inner in record.into_inner() {
                    if inner.as_rule() == Rule::SECTION {
                        let mut header = String::new();
                        let mut key_values = Vec::new();

                        for section_inner in inner.into_inner() {
                            match section_inner.as_rule() {
                                Rule::HEADER => {
                                    header = section_inner.as_str().trim_matches(['[', ']']).to_string();
                                }
                                Rule::KEY_VALUE => {
                                    let mut kv_inner = section_inner.into_inner();
                                    let key = kv_inner.next().unwrap().as_str().to_string();
                                    let value = kv_inner.next().unwrap().as_str().to_string();
                                    key_values.push(KeyValuePair { key, value });
                                }
                                _ => {}
                            }
                        }
                        sections.push(Section { header, key_values });
                    }
                }
            }
            _ => {
                println!("Unexpected rule: {:?}", record.as_rule());
            }
        }
    }
    Ok(SystemdService { sections })
}
