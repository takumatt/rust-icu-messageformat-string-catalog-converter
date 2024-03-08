use icu_messageformat_parser::{self, AstElement};
use linked_hash_map::LinkedHashMap;
use crate::xcstrings::{self, StringUnit, Substitution, VariationType};

pub struct XCStringSubstitutionBuilder {

}

impl XCStringSubstitutionBuilder {
    pub fn new() -> XCStringSubstitutionBuilder {
        XCStringSubstitutionBuilder {

        }
    }

    pub fn build(&self, plurals: Vec<AstElement>) -> LinkedHashMap<String, Substitution> {
        plurals.iter().enumerate().fold(LinkedHashMap::new(), |mut map, (index, plural)| {
            let pair = match plural {
                AstElement::Plural { value, plural_type, span, offset, options } => {
                    (
                        value, 
                        Substitution {
                            arg_num: index + 1,
                            format_specifier: "lld".to_string(),
                            variations: VariationType::Plural(options.0.iter().fold(LinkedHashMap::new(), |mut map, (key, value)| {
                                map.insert(key.to_string(), StringUnit {
                                    localization_state: xcstrings::LocalizationState::Translated,
                                    value: "%arg".to_string() // TODO: needs to be concatenated with the other literals?
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
