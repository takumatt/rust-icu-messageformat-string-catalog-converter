use std::collections::HashMap;

use crate::models::{self, ConverterOptions};
use crate::xcstring_formatter::{FormatterMode, XCStringFormatter};
use crate::xcstring_substitution_builder::XCStringSubstitutionBuilder;
use crate::xcstrings;
use icu_messageformat_parser::{self, Ast, AstElement};
use linked_hash_map::LinkedHashMap;

#[derive(Debug)]
pub struct XCStringConverter {
    source_language: String,
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
            let xcstring = self.convert_message(message.clone());
            xcstrings.strings.insert(message.key.clone(), xcstring);
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
        messages: LinkedHashMap<String, String>,
    ) -> LinkedHashMap<String, xcstrings::Localization> {
        let mut formatter = XCStringFormatter::new(FormatterMode::StringUnit);
        // TODO: Need to check the all arguments for plurals have the same type.
        LinkedHashMap::from_iter(messages.iter().map(|(locale, message)| {
            let mut parser = icu_messageformat_parser::Parser::new(message, &self.parser_options);
            let parsed = parser.parse().unwrap();
            let plurals: Vec<AstElement> = parsed
                .iter()
                .filter(|element| match element {
                    AstElement::Plural { .. } => true,
                    _ => false,
                })
                .cloned()
                .collect();
            let substitution_builder = XCStringSubstitutionBuilder::new();
            let substitutions = substitution_builder.build(plurals.clone());
            let formatted_strings = parsed
                .iter()
                .map(|element| formatter.format(element))
                .collect::<Vec<String>>()
                .join("");
            (
                locale.clone(),
                xcstrings::Localization {
                    string_unit: xcstrings::StringUnit {
                        localization_state: xcstrings::LocalizationState::Translated,
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
}

#[cfg(test)]
mod tests {
    use crate::models::ConverterOptions;
    use linked_hash_map::LinkedHashMap;

    #[test]
    fn test_convert() {
        let mut messages = LinkedHashMap::new();
        messages.insert("ja".to_string(), "こんにちは {your_name}、私は {my_name} です。".to_string());
        messages.insert("en".to_string(), "Hello {your_name}, I'm {my_name}.".to_string());
        messages.insert("ko".to_string(), "안녕하세요 {your_name}, 저는 {my_name} 입니다.".to_string());

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
    }
}
