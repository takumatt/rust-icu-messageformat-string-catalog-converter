use icu_messageformat_parser::{self, AstElement};
use linked_hash_map::LinkedHashMap;
use crate::xcstrings::{self, StringUnit, Substitution};

pub struct XCStringSubstitutionBuilder {

}

impl XCStringSubstitutionBuilder {
    pub fn new() -> XCStringSubstitutionBuilder {
        XCStringSubstitutionBuilder {

        }
    }

    pub fn build(&self, plurals: Vec<AstElement>) -> LinkedHashMap<String, Substitution> {
        plurals.iter().enumerate().fold(LinkedHashMap::new(), |mut map, (index, plural)| {
            let mut substitution = Substitution {
                arg_num: 0,
                format_specifier: "".to_string(),
                variations: LinkedHashMap::new(),
            };
            let mut arg_num = 0;
            let mut format_specifier = "".to_string();
            let mut variations = LinkedHashMap::new();
            let substitusions = match plural {
                AstElement::Plural { value, plural_type, span, offset, options } => {
                    Substitution {
                        arg_num: index,
                        format_specifier: format!("{}lld", index),
                        variations: LinkedHashMap::new(),
                    }
                },
                _ => { panic!("Unexpected AstElement") }
            };
            substitution.arg_num = arg_num;
            substitution.format_specifier = format_specifier;
            substitution.variations = variations;
            map.insert("".to_string(), substitution);
            map
        })
    }
}
