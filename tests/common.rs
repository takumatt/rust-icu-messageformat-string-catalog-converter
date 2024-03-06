use std::{fs, path::PathBuf};

use testing::{fixture, json};
use serde::Deserialize;
use rust_icu_messageformat_string_catalog_converter::models::LocalizableICUMessage;

#[derive(Debug, Deserialize)]
struct Fixture {
  message: String,
  options: String,
  output: String,
}

fn parse_fixture(file: PathBuf) -> Fixture {
  let contents = fs::read_to_string(file).unwrap();
  let sections: Vec<String> = contents.split("---\n").map(|s| s.to_string()).collect();
  Fixture {
    message: sections[0].clone(),
    options: sections[1].clone(),
    output: sections[2].clone(),
  }
}

#[fixture("tests/fixtures/basic")]
fn converter_tests(file: PathBuf) {
  let fixture_sections = parse_fixture(file);
  let message: LocalizableICUMessage = serde_json::from_str(&fixture_sections.message).unwrap();
  let options: icu_messageformat_parser::ParserOptions = serde_json::from_str(&fixture_sections.options).unwrap();
}
