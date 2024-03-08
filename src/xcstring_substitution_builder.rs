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

    pub fn build(&self, plurals: Vec<AstElement>) -> xcstrings::Substitution {

        let mut substitution = xcstrings::Substitution {
            arg_num: todo!(),
            format_specifier: todo!(),
            variations: todo!()
        };

        let variations: LinkedHashMap<String, StringUnit> = LinkedHashMap::from_iter(plurals.iter().map(|plural| {
            match plural {
                AstElement::Plural { value, plural_type, span, offset, options } => {
                    let value = value.to_string();
                    (value, StringUnit {
                        localization_state: todo!(),
                        value,
                    })
                },
                _ => panic!("Invalid plural element")
            }
        }));

        substitution

    }
}
