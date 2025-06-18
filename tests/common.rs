use std::{fs, path::PathBuf};

use rust_icu_messageformat_string_catalog_converter::models::{LocalizableICUMessage, LocalizableICUStrings};
use rust_icu_messageformat_string_catalog_converter::converter::XCStringConverter;
use testing::fixture;

#[fixture("tests/fixtures/simple_argument")]
#[fixture("tests/fixtures/pluralization")]
#[fixture("tests/fixtures/multiple_languages")]
#[fixture("tests/fixtures/icu_messageformat")]
#[fixture("tests/fixtures/multiple_arguments")]
#[fixture("tests/fixtures/select_splitting")]
fn converter_tests(dir: PathBuf) {
    let input_path = dir.join("input.json");
    let expected_path = dir.join("expected_output.json");
    let message = fs::read_to_string(&input_path).expect("Failed to read input.json");
    let output = fs::read_to_string(&expected_path).expect("Failed to read expected_output.json");

    // source_languageはinput.jsonやexpected_output.jsonから取得する必要がある場合はパースする
    // ここではexpected_output.jsonから取得する例
    let expected_json: serde_json::Value = serde_json::from_str(&output).expect("Failed to parse expected_output.json");
    let source_language = expected_json["sourceLanguage"].as_str().unwrap_or("").to_string();

    let messages: LocalizableICUStrings = serde_json::from_str(&message)
        .expect("Failed to parse test fixture message JSON");
    let converter = XCStringConverter::new(
        source_language,
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
