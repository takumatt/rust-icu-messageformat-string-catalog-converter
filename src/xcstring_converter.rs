use crate::models::{self, ConverterOptions};
use crate::xcstring_formatter::{FormatterMode, XCStringFormatter};
use crate::xcstring_substitution_builder::XCStringSubstitutionBuilder;
use crate::xcstrings;
use icu_messageformat_parser::{self, AstElement};
use linked_hash_map::LinkedHashMap;

#[derive(Debug)]
pub struct XCStringConverter {
    source_language: String,
    #[allow(dead_code)]
    converter_options: ConverterOptions,
    parser_options: icu_messageformat_parser::ParserOptions,
}

impl XCStringConverter {
    pub fn new(
        source_language: String,
        converter_options: ConverterOptions,
        parser_options: icu_messageformat_parser::ParserOptions,
    ) -> XCStringConverter {
        XCStringConverter {
            source_language: source_language,
            converter_options: converter_options,
            parser_options,
        }
    }

    pub fn convert(&self, messages: Vec<models::LocalizableICUMessage>) -> Result<xcstrings::XCStrings, String> {
        let mut xcstrings = xcstrings::XCStrings {
            source_language: self.source_language.clone(),
            strings: LinkedHashMap::new(),
            version: "1.0".to_string(),
        };
        
        for message in messages.iter() {
            // 変数の一貫性をチェック
            if let Err(error) = self.validate_variable_consistency(message) {
                return Err(error);
            }
            
            if self.has_select_elements(message) {
                if self.converter_options.split_select_elements {
                    // select要素を分割
                    let split_messages = self.split_select_message(message.clone());
                    for split_message in split_messages {
                        let xcstring = self.convert_message(split_message.clone());
                        xcstrings.strings.insert(split_message.key, xcstring);
                    }
                } else {
                    // エラーを返す
                    return Err(format!("Select elements are not supported by xcstrings. Found in key: '{}'. Consider enabling split_select_elements option.", message.key));
                }
            } else {
                // 通常処理
                let xcstring = self.convert_message(message.clone());
                xcstrings.strings.insert(message.key.clone(), xcstring);
            }
        }
        
        Ok(xcstrings)
    }

    fn validate_variable_consistency(&self, message: &models::LocalizableICUMessage) -> Result<(), String> {
        let mut reference_variables: Option<std::collections::HashSet<String>> = None;
        
        for (locale, msg_value) in &message.messages {
            let variables = self.extract_variables(&msg_value.value)?;
            
            match &reference_variables {
                None => {
                    // 最初の言語の変数セットを基準とする
                    reference_variables = Some(variables);
                }
                Some(ref_vars) => {
                    // 変数の数と名前が一致するかチェック
                    if variables.len() != ref_vars.len() {
                        return Err(format!(
                            "Variable count mismatch in key '{}'. Language '{}' has {} variables, but expected {}",
                            message.key, locale, variables.len(), ref_vars.len()
                        ));
                    }
                    
                    // 変数名が一致するかチェック
                    for var in &variables {
                        if !ref_vars.contains(var) {
                            return Err(format!(
                                "Variable name mismatch in key '{}'. Language '{}' contains variable '{}' which is not found in other languages. Expected variables: {:?}",
                                message.key, locale, var, ref_vars.iter().collect::<Vec<_>>()
                            ));
                        }
                    }
                    
                    for var in ref_vars {
                        if !variables.contains(var) {
                            return Err(format!(
                                "Missing variable in key '{}'. Language '{}' is missing variable '{}' which exists in other languages",
                                message.key, locale, var
                            ));
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn extract_variables(&self, message_value: &str) -> Result<std::collections::HashSet<String>, String> {
        let mut variables = std::collections::HashSet::new();
        let mut parser = icu_messageformat_parser::Parser::new(message_value, &self.parser_options);
        
        match parser.parse() {
            Ok(parsed) => {
                self.collect_variables_from_ast(&parsed, &mut variables);
                Ok(variables)
            }
            Err(e) => Err(format!("Failed to parse message '{}': {:?}", message_value, e))
        }
    }
    
    fn collect_variables_from_ast(&self, elements: &[icu_messageformat_parser::AstElement], variables: &mut std::collections::HashSet<String>) {
        for element in elements {
            match element {
                icu_messageformat_parser::AstElement::Argument { value, .. } |
                icu_messageformat_parser::AstElement::Number { value, .. } |
                icu_messageformat_parser::AstElement::Date { value, .. } => {
                    variables.insert(value.clone());
                }
                icu_messageformat_parser::AstElement::Plural { value, options, .. } => {
                    variables.insert(value.clone());
                    // plural/selectの内部要素からも変数を収集
                    for (_, option) in &options.0 {
                        self.collect_variables_from_ast(&option.value, variables);
                    }
                }
                icu_messageformat_parser::AstElement::Select { value, options, .. } => {
                    variables.insert(value.clone());
                    // plural/selectの内部要素からも変数を収集
                    for (_, option) in &options.0 {
                        self.collect_variables_from_ast(&option.value, variables);
                    }
                }
                _ => {}
            }
        }
    }

    fn convert_message(
        &self,
        localizable_icu_message: models::LocalizableICUMessage,
    ) -> xcstrings::XCString {
        let mut xcstring = xcstrings::XCString {
            extraction_state: xcstrings::ExtractionState::Manual,
            localizations: LinkedHashMap::new(),
        };
        self.format(localizable_icu_message.messages)
            .iter()
            .for_each(|(locale, localization)| {
                xcstring
                    .localizations
                    .insert(locale.clone(), localization.clone());
            });
        xcstring
    }

    fn format(
        &self,
        messages: LinkedHashMap<String, models::LocalizableICUMessageValue>,
    ) -> LinkedHashMap<String, xcstrings::Localization> {
        let mut formatter = XCStringFormatter::new(FormatterMode::StringUnit);
        // TODO: Need to check the all arguments for plurals have the same type.
        LinkedHashMap::from_iter(messages.iter().map(|(locale, message)| {
            let mut parser = icu_messageformat_parser::Parser::new(&message.value, &self.parser_options);
            let parsed = parser.parse().unwrap();
            let plural_and_selects: Vec<AstElement> = parsed
                .iter()
                .filter(|element| matches!(element, AstElement::Plural { .. } | AstElement::Select { .. }))
                .cloned()
                .collect();
            let substitution_builder = XCStringSubstitutionBuilder::new();
            let substitutions = substitution_builder.build(plural_and_selects.clone());
            let formatted_strings = parsed
                .iter()
                .map(|element| formatter.format(element))
                .collect::<Vec<String>>()
                .join("")
                .trim()
                .to_string();
            (
                locale.clone(),
                xcstrings::Localization {
                    string_unit: xcstrings::StringUnit {
                        localization_state: match message.state.as_str() {
                            "translated" => xcstrings::LocalizationState::Translated,
                            "needs_review" => xcstrings::LocalizationState::NeedsReview,
                            _ => xcstrings::LocalizationState::Translated,
                        },
                        value: formatted_strings,
                    },
                    substitutions: if substitutions.is_empty() {
                        None
                    } else {
                        Some(substitutions)
                    },
                },
            )
        }))
    }

    fn has_select_elements(&self, message: &models::LocalizableICUMessage) -> bool {
        message.messages.values().any(|msg_value| {
            let mut parser = icu_messageformat_parser::Parser::new(&msg_value.value, &self.parser_options);
            if let Ok(parsed) = parser.parse() {
                parsed.iter().any(|element| matches!(element, AstElement::Select { .. }))
            } else {
                false
            }
        })
    }

    fn split_select_message(&self, message: models::LocalizableICUMessage) -> Vec<models::LocalizableICUMessage> {
        let mut split_messages = Vec::new();
        
        // まず、最初の言語のselect要素を解析して、分割する選択肢を取得
        let first_message = message.messages.values().next().unwrap();
        let mut parser = icu_messageformat_parser::Parser::new(&first_message.value, &self.parser_options);
        if let Ok(parsed) = parser.parse() {
            if let Some(select_element) = parsed.iter().find(|element| matches!(element, AstElement::Select { .. })) {
                if let AstElement::Select { value: _, span: _, options } = select_element {
                    for (case_key, _) in &options.0 {
                        let new_key = format!("{}_{}", message.key, case_key);
                        let mut new_messages = LinkedHashMap::new();
                        
                        for (locale, msg_value) in &message.messages {
                            let new_value = self.replace_select_with_case(&msg_value.value, case_key);
                            new_messages.insert(locale.clone(), models::LocalizableICUMessageValue {
                                value: new_value,
                                state: msg_value.state.clone(),
                            });
                        }
                        
                        split_messages.push(models::LocalizableICUMessage {
                            key: new_key,
                            messages: new_messages,
                            comment: message.comment.clone(),
                        });
                    }
                }
            }
        }
        
        if split_messages.is_empty() {
            vec![message]
        } else {
            split_messages
        }
    }

    fn replace_select_with_case(&self, original_value: &str, case_key: &str) -> String {
        let mut parser = icu_messageformat_parser::Parser::new(original_value, &self.parser_options);
        if let Ok(parsed) = parser.parse() {
            let mut formatter = XCStringFormatter::new(FormatterMode::StringUnit);
            let replaced_elements: Vec<String> = parsed.iter().map(|element| {
                match element {
                    AstElement::Select { options, .. } => {
                        // この言語のselect要素から対応するケースを見つけて置換する
                        let case_option = options.0.iter().find(|(key, _)| *key == case_key);
                        if let Some((_, option)) = case_option {
                            option.value.iter().map(|e| formatter.format(e)).collect::<Vec<String>>().join("")
                        } else {
                            // フォールバック: "other"ケースを探す
                            let other_option = options.0.iter().find(|(key, _)| *key == "other");
                            if let Some((_, option)) = other_option {
                                option.value.iter().map(|e| formatter.format(e)).collect::<Vec<String>>().join("")
                            } else {
                                "".to_string()
                            }
                        }
                    },
                    _ => formatter.format(element)
                }
            }).collect();
            replaced_elements.join("").trim().to_string()
        } else {
            original_value.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{ConverterOptions, LocalizableICUMessageValue};
    use linked_hash_map::LinkedHashMap;

    #[test]
    fn test_convert() {
        let mut messages = LinkedHashMap::new();
        messages.insert(
            "ja".to_string(),
            LocalizableICUMessageValue {
                value: "こんにちは {your_name}、私は {my_name} です。".to_string(),
                state: "translated".to_string(),
            },
        );
        messages.insert(
            "en".to_string(),
            LocalizableICUMessageValue {
                value: "Hello {your_name}, I'm {my_name}.".to_string(),
                state: "needs_review".to_string(),
            },
        );
        messages.insert(
            "ko".to_string(),
            LocalizableICUMessageValue {
                value: "안녕하세요 {your_name}, 저는 {my_name} 입니다.".to_string(),
                state: "translated".to_string(),
            },
        );

        let message = super::models::LocalizableICUMessage {
            key: "hello".to_string(),
            messages,
            comment: Some("A greeting message with both the user's name and speaker's name".to_string()),
        };
        let converter = super::XCStringConverter::new(
            "ja".to_string(),
            ConverterOptions::default(),
            icu_messageformat_parser::ParserOptions::default(),
        );
        let xcstrings = converter.convert(vec![message]).unwrap();
        let xcstring = xcstrings.strings.get("hello").unwrap();
        assert_eq!(xcstring.localizations.len(), 3);
        assert_eq!(
            xcstring.localizations.get("ja").unwrap().string_unit.value,
            "こんにちは %1$@、私は %2$@ です。"
        );
        assert_eq!(
            xcstring.localizations.get("en").unwrap().string_unit.value,
            "Hello %1$@, I'm %2$@."
        );
        assert_eq!(
            xcstring.localizations.get("ko").unwrap().string_unit.value,
            "안녕하세요 %1$@, 저는 %2$@ 입니다."
        );
        assert_eq!(
            xcstring.localizations.get("ja").unwrap().string_unit.localization_state,
            super::xcstrings::LocalizationState::Translated
        );
        assert_eq!(
            xcstring.localizations.get("en").unwrap().string_unit.localization_state,
            super::xcstrings::LocalizationState::NeedsReview
        );
        assert_eq!(
            xcstring.localizations.get("ko").unwrap().string_unit.localization_state,
            super::xcstrings::LocalizationState::Translated
        );
    }

    #[test]
    fn test_variable_count_mismatch() {
        let mut messages = LinkedHashMap::new();
        messages.insert(
            "en".to_string(),
            LocalizableICUMessageValue {
                value: "Hello {name} and {age}!".to_string(),
                state: "translated".to_string(),
            },
        );
        messages.insert(
            "ja".to_string(),
            LocalizableICUMessageValue {
                value: "こんにちは {name} さん！".to_string(),
                state: "translated".to_string(),
            },
        );

        let message = super::models::LocalizableICUMessage {
            key: "inconsistent_test".to_string(),
            messages,
            comment: Some("Test case with inconsistent variable count".to_string()),
        };
        let converter = super::XCStringConverter::new(
            "en".to_string(),
            ConverterOptions::default(),
            icu_messageformat_parser::ParserOptions::default(),
        );
        let result = converter.convert(vec![message]);
        assert!(result.is_err());
        let error_message = result.unwrap_err();
        assert!(error_message.contains("Variable count mismatch"));
        assert!(error_message.contains("inconsistent_test"));
    }

    #[test]
    fn test_variable_name_mismatch() {
        let mut messages = LinkedHashMap::new();
        messages.insert(
            "en".to_string(),
            LocalizableICUMessageValue {
                value: "Hello {firstName}!".to_string(),
                state: "translated".to_string(),
            },
        );
        messages.insert(
            "ja".to_string(),
            LocalizableICUMessageValue {
                value: "こんにちは {lastName} さん！".to_string(),
                state: "translated".to_string(),
            },
        );

        let message = super::models::LocalizableICUMessage {
            key: "wrong_variable_names".to_string(),
            messages,
            comment: Some("Test case with different variable names".to_string()),
        };
        let converter = super::XCStringConverter::new(
            "en".to_string(),
            ConverterOptions::default(),
            icu_messageformat_parser::ParserOptions::default(),
        );
        let result = converter.convert(vec![message]);
        assert!(result.is_err());
        let error_message = result.unwrap_err();
        assert!(error_message.contains("Variable name mismatch"));
        assert!(error_message.contains("lastName"));
        assert!(error_message.contains("firstName"));
    }
}

