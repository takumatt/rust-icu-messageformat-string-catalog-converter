use std::fmt::Formatter;

use icu_messageformat_parser::{self, AstElement};
use linked_hash_map::LinkedHashMap;
use crate::{xcstring_formatter::{XCStringFormatter, FormatterMode}, xcstrings::{self, StringUnit, Substitution, VariationType, VariationValue}};

pub struct XCStringSubstitutionBuilder {

}

impl XCStringSubstitutionBuilder {
    pub fn new() -> XCStringSubstitutionBuilder {
        XCStringSubstitutionBuilder {

        }
    }

    pub fn build(&self, plurals: Vec<AstElement>) -> LinkedHashMap<String, Substitution> {
        let mut formatter = XCStringFormatter::new(FormatterMode::Plural);
        plurals.iter().enumerate().fold(LinkedHashMap::new(), |mut map, (index, plural)| {
            let pair = match plural {
                AstElement::Plural { value, plural_type, span, offset, options } => {
                    (
                        value, 
                        Substitution {
                            arg_num: index + 1,
                            format_specifier: "lld".to_string(),
                            variations: VariationType::Plural(options.0.iter().fold(LinkedHashMap::new(), |mut map, (key, value)| {
                                let formatted_strings = value.value
                                    .iter()
                                    .map(|element| formatter.format(element))
                                    .collect::<Vec<String>>()
                                    .join("");
                                println!("formatted_strings: {:?}", formatted_strings);
                                map.insert(key.to_string(), VariationValue {                                    
                                    string_unit: StringUnit {
                                        localization_state: xcstrings::LocalizationState::Translated,
                                        value: formatted_strings
                                    },
                                });
                                map
                            }))
                        }
                    )
                },
                _ => { panic!("Unexpected AstElement") }
            };
            map.insert(pair.0.clone(), pair.1);
            map
        })
    }
}
