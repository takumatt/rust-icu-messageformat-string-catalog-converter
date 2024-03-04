pub use std::collections::HashMap;
use icu_messageformat_parser::AstElement;

#[derive(Debug)]
pub struct LocalizableICUMessage {
   pub key: String,
   pub messages: HashMap<String, String>,
}

#[derive(Debug)]
pub struct XCStringFormatter<'a> {
   pub element_type: AstElement<'a>,
}

impl<'a> XCStringFormatter<'a> {
   pub fn new(element: &'a AstElement) -> Self {
      Self {
         element_type: element.clone(),
      }
   }

   pub fn format(&self) -> String {
      match &self.element_type {
         AstElement::Literal { value, span } => value.to_string(),
         AstElement::Argument { value, span } => value.to_string(),
         _ => "".to_string(),
      }
   }
}
