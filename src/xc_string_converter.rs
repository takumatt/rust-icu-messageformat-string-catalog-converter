use std::{collections::HashMap, fmt::format, hash::Hash, vec};

use icu_messageformat_parser::{self, AstElement};
use linked_hash_map::LinkedHashMap;
use crate::xcstrings::{self, XCString};
use crate::models::{self, LocalizableICUMessage};

#[derive(Debug)]
pub struct XCStringConverter {
    source_language: String,
    parser_options: icu_messageformat_parser::ParserOptions,
}

impl XCStringConverter {
    pub fn new(
        source_language: String,
        parser_options: icu_messageformat_parser::ParserOptions
    ) -> XCStringConverter {
        XCStringConverter {
            source_language: source_language,
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

    fn convert_message(&self, localizable_icu_message: models::LocalizableICUMessage) -> xcstrings::XCString {
        let mut xcstring = xcstrings::XCString {
            extraction_state: xcstrings::ExtractionState::Manual,
            localizations: LinkedHashMap::new(),
        };
        // TODO: Formatter should do this
        // let mut vec: Vec<(String, String)> = localizable_icu_message.messages.into_iter().collect();
        // if let Some(index) = vec.iter().position(|(locale, _)| locale == &self.source_language) {
        //     vec.swap(0, index)
        // }
        self.format(localizable_icu_message.messages).iter().for_each(|(locale, localization)| {
            xcstring.localizations.insert(locale.clone(), localization.clone());
        });
        xcstring
    }

    fn format(&self, messages: LinkedHashMap<String, String>) -> LinkedHashMap<String, xcstrings::Localization> {
        let mut formatter = models::XCStringFormatter::new(self.source_language.clone());
        LinkedHashMap::from_iter(messages.iter().map(|(locale, message)| {
            let mut parser = icu_messageformat_parser::Parser::new(message, &self.parser_options);
            let parsed = parser.parse().unwrap();
            let formatted_strings = parsed.iter().map(|element| {
                formatter.format(element)
            }).collect::<Vec<String>>().join(""); 
            println!("formatted_strings: {:?}", formatted_strings);           
            (
                locale.clone(),
                xcstrings::Localization {
                    string_unit: xcstrings::StringUnit {
                        localization_state: xcstrings::LocalizationState::Translated,
                        value: formatted_strings,
                    },
                }
            )
        }))
    }

    fn _format_with_plural(&self, messages: LinkedHashMap<String, String>) -> LinkedHashMap<String, xcstrings::Localization> {
        LinkedHashMap::from_iter(messages.iter().map(|(locale, message)| {
            let mut parser = icu_messageformat_parser::Parser::new(message, &self.parser_options);
            let ast = parser.parse().unwrap();
            ast.iter().for_each(|element| {
                match element {
                    AstElement::Argument { value, span } => {
                        println!("value: {:?}, span: {:?}", value, span);
                    },
                    others => {
                        println!("others: {:?}", others);
                    }
                }
            });
            (
                locale.clone(),
                xcstrings::Localization {
                    string_unit: xcstrings::StringUnit {
                        localization_state: xcstrings::LocalizationState::Translated,
                        value: message.clone(),
                    },
                }
            )
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::xcstrings;

    #[test]
    fn test_convert() {
        let message = super::models::LocalizableICUMessage {
            key: "key".to_string(),
            messages: vec![
                ("en".to_string(), "Hello, {name1} and {name2}!".to_string()),
                ("es".to_string(), "¡Hola, {name2} y {name1}!".to_string()),
            ].into_iter().collect()
        };
        let converter = super::XCStringConverter::new(
            "en".to_string(),
            icu_messageformat_parser::ParserOptions::default()
        );
        let xcstrings = converter.convert(vec![message]);
        let xcstring = xcstrings.strings.get("key").unwrap();
        assert_eq!(xcstring.localizations.len(), 2);
        assert_eq!(xcstring.localizations.get("en").unwrap().string_unit.value, "Hello, %1$@ and %2$@!");
        assert_eq!(xcstring.localizations.get("es").unwrap().string_unit.value, "¡Hola, %2$@ y %1$@!");
    }
}