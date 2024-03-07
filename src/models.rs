pub use std::collections::HashMap;
pub use std::collections::BTreeSet;
use std::default;
use linked_hash_map;
use icu_messageformat_parser::AstElement;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct LocalizableICUMessage {
   pub key: String,
   // #[serde(flatten)]
   pub messages: linked_hash_map::LinkedHashMap<String, String>,
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

#[cfg(test)]
mod test {
    use icu_messageformat_parser::AstElement;

   #[test]
   fn test_format() {
      let mut formatter = super::XCStringFormatter::new("en".to_string());
      let element = AstElement::Argument { value: "name1".to_string(), span: None };
      assert_eq!(formatter.format(&element), "%1$@");
      let element = AstElement::Literal { value: "Hello, ".to_string(), span: None };
      assert_eq!(formatter.format(&element), "Hello, ");
      let element = AstElement::Argument { value: "name2".to_string(), span: None };
      assert_eq!(formatter.format(&element), "%2$@");
   }

   #[test]
   fn test_deserialize_localizable_icu_message() {
      let json = r#"{
         "key": "key",
         "messages": {
            "en": "Hello, {name1} and {name2}!",
            "es": "¡Hola, {name2} y {name1}!"
         }
      }"#;
      let message: super::LocalizableICUMessage = serde_json::from_str(json).unwrap();
      assert_eq!(message.key, "key");
      assert_eq!(message.messages.len(), 2);
      assert_eq!(message.messages.get("en").unwrap(), "Hello, {name1} and {name2}!");
      assert_eq!(message.messages.get("es").unwrap(), "¡Hola, {name2} y {name1}!");
   }
}