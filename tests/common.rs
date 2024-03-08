use std::{fs, path::PathBuf};

use rust_icu_messageformat_string_catalog_converter::models::LocalizableICUMessage;
use rust_icu_messageformat_string_catalog_converter::xcstring_converter::XCStringConverter;
use serde::Deserialize;
use testing::fixture;

#[derive(Debug, Deserialize)]
struct Fixture {
    message: String,
    options: String,
    output: String,
}

#[derive(Debug, Deserialize)]
struct FixtureOptions {
    source_language: String,
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
#[fixture("tests/fixtures/plural")]
fn converter_tests(file: PathBuf) {
    let fixture_sections = parse_fixture(file);
    let message: LocalizableICUMessage = serde_json::from_str(&fixture_sections.message).unwrap();
    let options: FixtureOptions = serde_json::from_str(&fixture_sections.options).unwrap();
    let output: String = fixture_sections.output;
    let converter = XCStringConverter::new(
        options.source_language,
        icu_messageformat_parser::ParserOptions::default(),
    );
    let result = converter.convert(vec![message]);
    let result_json_string = serde_json::to_string_pretty(&result).unwrap();
    similar_asserts::assert_eq!(result_json_string, output);
}
