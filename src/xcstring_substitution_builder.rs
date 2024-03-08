use icu_messageformat_parser::{self, AstElement};
use crate::xcstrings;

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
            variations: todo!(),
        };
        substitution  
    }
}
