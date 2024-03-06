pub use std::collections::HashMap;
pub use std::collections::BTreeSet;
use std::default;
use icu_messageformat_parser::AstElement;

#[derive(Debug)]
pub struct LocalizableICUMessage {
   pub key: String,
   pub messages: Vec<(String, String)>,
}

#[derive(Debug)]
pub struct XCStringFormatter {
   source_language: String,
   argument_positions: BTreeSet<String>
}

impl<'a> XCStringFormatter {
   pub fn new(
      source_language: String,
   ) -> Self {
      XCStringFormatter {
         source_language: "".to_string(),
         argument_positions: BTreeSet::new(),
      }
   }

   pub fn format(&mut self, element: &'a AstElement) -> String {
      match &element {
         AstElement::Literal { value, span } => value.to_string(),
         AstElement::Argument { value, span } => {
            self.argument_positions.insert(value.to_string());
            let index = self.argument_positions.iter().position(|x| x == value).unwrap();
            format!("%{}$@", index + 1)
         },
         AstElement::Number { value, span, style } => {
            self.argument_positions.insert(value.to_string());
            let index = self.argument_positions.iter().position(|x| x == value).unwrap();
            format!("%{}$lld", index)
         },
         _ => "".to_string(),
      }
   }
}
