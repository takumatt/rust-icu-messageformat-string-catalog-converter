use crate::{
    xcstring_formatter::{FormatterMode, XCStringFormatter},
    xcstrings::{self, StringUnit, Substitution, VariationType, VariationValue},
};
use icu_messageformat_parser::{self, AstElement};
use linked_hash_map::LinkedHashMap;

pub struct XCStringSubstitutionBuilder {}

impl XCStringSubstitutionBuilder {
    pub fn new() -> XCStringSubstitutionBuilder {
        XCStringSubstitutionBuilder {}
    }

    pub fn build(&self, elements: Vec<AstElement>) -> LinkedHashMap<String, Substitution> {
        let mut formatter = XCStringFormatter::new(FormatterMode::Plural);
        elements
            .iter()
            .enumerate()
            .fold(LinkedHashMap::new(), |mut map, (index, element)| {
                match element {
                    AstElement::Plural {
                        value,
                        plural_type: _,
                        span: _,
                        offset: _,
                        options,
                    } => {
                        map.insert(
                            value.clone(),
                            Substitution {
                                arg_num: index + 1,
                                format_specifier: "lld".to_string(),
                                variations: VariationType::Plural(options.0.iter().fold(
                                    LinkedHashMap::new(),
                                    |mut map, (key, value)| {
                                        let formatted_strings = value
                                            .value
                                            .iter()
                                            .map(|element| formatter.format(element))
                                            .collect::<Vec<String>>()
                                            .join("");
                                        map.insert(
                                            KeyFormat::from_string(&key.to_string()).to_string(),
                                            VariationValue {
                                                string_unit: StringUnit {
                                                    localization_state:
                                                        xcstrings::LocalizationState::Translated,
                                                    value: formatted_strings,
                                                },
                                            },
                                        );
                                        map
                                    },
                                )),
                            },
                        );
                    }
                    AstElement::Select {
                        value,
                        options,
                        ..
                    } => {
                        map.insert(
                            value.clone(),
                            Substitution {
                                arg_num: index + 1,
                                format_specifier: "@".to_string(),
                                variations: VariationType::Select(options.0.iter().fold(
                                    LinkedHashMap::new(),
                                    |mut map, (key, value)| {
                                        let formatted_strings = value
                                            .value
                                            .iter()
                                            .map(|element| formatter.format(element))
                                            .collect::<Vec<String>>()
                                            .join("");
                                        map.insert(
                                            key.to_string(),
                                            VariationValue {
                                                string_unit: StringUnit {
                                                    localization_state:
                                                        xcstrings::LocalizationState::Translated,
                                                    value: formatted_strings,
                                                },
                                            },
                                        );
                                        map
                                    },
                                )),
                            },
                        );
                    }
                    _ => {}
                }
                map
            })
    }
}

enum KeyFormat {
    Zero,
    One,
    Other,
}

impl KeyFormat {
    fn from_string(key: &String) -> KeyFormat {
        match key.as_str() {
            "zero" => KeyFormat::Zero,
            "one" => KeyFormat::One,
            "two" => panic!("String Catalog doesn't support custom keys starting with 'two'"),
            "few" => panic!("String Catalog doesn't support custom keys starting with 'few'"),
            "many" => panic!("String Catalog doesn't support custom keys starting with 'many'"),
            "other" => KeyFormat::Other,
            "=0" => KeyFormat::Zero, // "zero" is an alias for "=0"
            "=1" => KeyFormat::One,  // "one" is an alias for "=1"
            _ => {
                if key.as_str().starts_with("=") {
                    panic!("String Catalog doesn't support custom keys starting with '=' except for zero and one")
                }
                panic!("Unexpected key format")
            }
        }
    }

    fn to_string(&self) -> String {
        match self {
            KeyFormat::Zero => "zero".to_string(),
            KeyFormat::One => "one".to_string(),
            KeyFormat::Other => "other".to_string(),
        }
    }
}
