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

    pub fn convert(&self, messages: Vec<models::LocalizableICUMessage>) -> xcstrings::XCStrings {
        let mut xcstrings = xcstrings::XCStrings {
            source_language: self.source_language.clone(),
            strings: LinkedHashMap::new(),
            version: "1.0".to_string(),
        };
        
        messages.iter().for_each(|message| {
            if self.converter_options.split_select_elements && self.has_select_elements(message) {
                let split_messages = self.split_select_message(message.clone());
                for split_message in split_messages {
                    let xcstring = self.convert_message(split_message.clone());
                    xcstrings.strings.insert(split_message.key, xcstring);
                }
            } else {
                let xcstring = self.convert_message(message.clone());
                xcstrings.strings.insert(message.key.clone(), xcstring);
            }
        });
        xcstrings
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
                    for (case_key, case_option) in &options.0 {
                        let new_key = format!("{}_{}", message.key, case_key);
                        let mut new_messages = LinkedHashMap::new();
                        
                        for (locale, msg_value) in &message.messages {
                            let new_value = self.replace_select_with_case(&msg_value.value, case_key, &case_option.value);
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

    fn replace_select_with_case(&self, original_value: &str, _case_key: &str, case_value: &[AstElement]) -> String {
        let mut parser = icu_messageformat_parser::Parser::new(original_value, &self.parser_options);
        if let Ok(parsed) = parser.parse() {
            let mut formatter = XCStringFormatter::new(FormatterMode::StringUnit);
            let replaced_elements: Vec<String> = parsed.iter().map(|element| {
                match element {
                    AstElement::Select { .. } => {
                        // select要素を選択されたケースの値に置き換える
                        case_value.iter().map(|e| formatter.format(e)).collect::<Vec<String>>().join("")
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
        let xcstrings = converter.convert(vec![message]);
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
}
