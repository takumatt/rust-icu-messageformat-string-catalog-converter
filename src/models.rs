pub use std::collections::HashMap;
use icu_messageformat_parser::AstElement;

#[derive(Debug)]
pub struct LocalizableICUMessage {
   pub key: String,
   pub messages: HashMap<String, String>,
}

#[derive(Debug)]
pub struct XCStringFormatter {
   
}

impl XCStringFormatter {
   pub fn new(element: &AstElement) -> Self {
      Self {
         
      }
   }
}
