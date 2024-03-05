use icu_messageformat_parser;
use crate::{models, xcstring::{self, XCString}};

#[derive(Debug)]
pub struct XCStringConverter {
    parser_options: icu_messageformat_parser::ParserOptions,
}

impl XCStringConverter {
    pub fn new(
        parser_options: icu_messageformat_parser::ParserOptions
    ) -> XCStringConverter {
        XCStringConverter {
            parser_options,
        }
    }

    pub fn convert(&self, localizable_icu_message: models::LocalizableICUMessage) -> XCString {
        let mut xcstring = xcstring::XCString {
            extraction_state: xcstring::ExtractionState::Manual,
            localizations: std::collections::HashMap::new(),
        };
        let mut formatter = models::XCStringFormatter::new();
        localizable_icu_message.messages.iter().for_each(|(locale, message)| {
            let mut parser = icu_messageformat_parser::Parser::new(message, &self.parser_options);
            let parsed = parser.parse().unwrap();
            let formatted_strings = parsed.iter().map(|element| {
                formatter.format(element)
            }).collect::<Vec<String>>().join("");
            xcstring.localizations.insert(locale.clone(), xcstring::Localization {
                string_unit: xcstring::StringUnit {
                    localization_state: xcstring::LocalizationState::Translated,
                    value: formatted_strings,
                },
            });
        });
        xcstring
    }
}