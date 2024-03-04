use icu_messageformat_parser;
use crate::models;

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

    pub fn convert(&self, localizable_icu_message: models::LocalizableICUMessage) -> String {
        let parsed_messages: Vec<_> = localizable_icu_message.messages.iter().map(|(locale, message)| {
            let mut parser = icu_messageformat_parser::Parser::new(message, &self.parser_options);
            let parsed = parser.parse().unwrap();
            parsed.iter().for_each(|element| {
                let formatted = models::XCStringFormatter::new(element).format(); // Dereference the element reference
                println!("{:?}{:?}", locale, formatted);
            });
        }).collect();

        parsed_messages.iter().for_each(|parsed_message| {

        });
        
        "".to_string()
    }
}