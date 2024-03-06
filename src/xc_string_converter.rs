use std::{collections::HashMap, fmt::format, hash::Hash};

use icu_messageformat_parser;
use crate::{models::{self, LocalizableICUMessage}, xcstring::{self, Localization, XCString}};

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

    pub fn convert(&self, localizable_icu_message: models::LocalizableICUMessage) -> XCString {
        let mut xcstring = xcstring::XCString {
            extraction_state: xcstring::ExtractionState::Manual,
            localizations: std::collections::HashMap::new(),
        };
        self.format(localizable_icu_message.messages.clone()).iter().for_each(|(locale, localization)| {
            xcstring.localizations.insert(locale.clone(), localization.clone());
        });
        xcstring
    }

    fn format(&self, messages: Vec<(String, String)>) -> HashMap<String, Localization> {
        let mut formatter = models::XCStringFormatter::new(self.source_language.clone());
        HashMap::from_iter(messages.iter().map(|(locale, message)| {
            let mut parser = icu_messageformat_parser::Parser::new(message, &self.parser_options);
            let parsed = parser.parse().unwrap();
            let formatted_strings = parsed.iter().map(|element| {
                formatter.format(element)
            }).collect::<Vec<String>>().join(""); 
            println!("formatted_strings: {:?}", formatted_strings);           
            (
                locale.clone(),
                xcstring::Localization {
                    string_unit: xcstring::StringUnit {
                        localization_state: xcstring::LocalizationState::Translated,
                        value: formatted_strings,
                    },
                }
            )
        }))
    }
}