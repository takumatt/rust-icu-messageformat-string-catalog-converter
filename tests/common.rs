use std::{fs, path::PathBuf};

use rust_icu_messageformat_string_catalog_converter::models::{LocalizableICUMessage, LocalizableICUStrings};
use rust_icu_messageformat_string_catalog_converter::converter::XCStringConverter;
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
    let contents = fs::read_to_string(file).expect("Failed to read test fixture file");
    let sections: Vec<String> = contents.split("---\n").map(|s| s.to_string()).collect();
    if sections.len() < 3 {
        panic!("Test fixture must have 3 sections separated by '---\\n'");
    }
    Fixture {
        message: sections[0].clone(),
        options: sections[1].clone(),
        output: sections[2].clone(),
    }
}

#[fixture("tests/fixtures/simple_argument")]
#[fixture("tests/fixtures/pluralization")]
#[fixture("tests/fixtures/multiple_languages")]
#[fixture("tests/fixtures/icu_messageformat")]
#[fixture("tests/fixtures/multiple_arguments")]
#[fixture("tests/fixtures/select_splitting")]
fn converter_tests(file: PathBuf) {
    let fixture_sections = parse_fixture(file);
    let messages: LocalizableICUStrings = serde_json::from_str(&fixture_sections.message)
        .expect("Failed to parse test fixture message JSON");
    let options: FixtureOptions = serde_json::from_str(&fixture_sections.options)
        .expect("Failed to parse test fixture options JSON");
    let output: String = fixture_sections.output;
    let converter = XCStringConverter::new(
        options.source_language,
        rust_icu_messageformat_string_catalog_converter::models::ConverterOptions::default(),
        icu_messageformat_parser::ParserOptions::default(),
    );
    let messages: Vec<LocalizableICUMessage> = messages.strings.into_iter().map(|s| s.into()).collect();
    let result = converter.convert(messages).expect("Failed to convert messages");
    let result_json_string = serde_json::to_string_pretty(&result)
        .expect("Failed to serialize result to JSON")
        .trim()
        .to_string();
    similar_asserts::assert_eq!(result_json_string, output.trim().to_string());
}

#[test]
fn test_select_error_case() {
    let messages_json = r#"{
        "strings": [
            {
                "key": "user_status",
                "messages": {
                    "en": { "value": "{gender, select, male {He} female {She} other {They}} is online.", "state": "translated" }
                },
                "comment": "User online status with gender selection"
            }
        ]
    }"#;
    
    let messages: LocalizableICUStrings = serde_json::from_str(messages_json)
        .expect("Failed to parse test JSON");
    let mut options = rust_icu_messageformat_string_catalog_converter::models::ConverterOptions::default();
    options.split_select_elements = false;
    
    let converter = XCStringConverter::new(
        "en".to_string(),
        options,
        icu_messageformat_parser::ParserOptions::default(),
    );
    
    let messages: Vec<LocalizableICUMessage> = messages.strings.into_iter().map(|s| s.into()).collect();
    let result = converter.convert(messages);
    
    assert!(result.is_err());
    let error_message = result.unwrap_err();
    println!("Error message: {}", error_message);
    assert!(error_message.contains("Select elements are not supported"));
    assert!(error_message.contains("user_status"));
}
