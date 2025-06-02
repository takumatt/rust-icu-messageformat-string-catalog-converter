use icu_messageformat_parser::{self, AstElement};
use linked_hash_set::LinkedHashSet;

#[derive(Debug)]
pub enum FormatterMode {
    StringUnit,
    Plural,
}

pub struct XCStringFormatter {
    formatter_mode: FormatterMode,
    argument_positions: LinkedHashSet<String>,
}

impl XCStringFormatter {
    pub fn new(mode: FormatterMode) -> Self {
        XCStringFormatter {
            formatter_mode: mode,
            argument_positions: LinkedHashSet::new(),
        }
    }

    pub fn format(&mut self, element: &AstElement) -> Result<String, String> {
        match &element {
            AstElement::Literal { value, .. } => Ok(value.to_string()),
            AstElement::Argument { value, .. } => {
                if !self.argument_positions.contains(value) {
                    self.argument_positions.insert(value.to_string());
                }
                let index = self
                    .argument_positions
                    .iter()
                    .position(|x| x.eq(value))
                    .ok_or_else(|| format!("Failed to find position for argument '{}'", value))?;
                let position = index.checked_add(1)
                    .ok_or_else(|| format!("Position overflow for argument '{}'", value))?;
                match self.formatter_mode {
                    FormatterMode::StringUnit => Ok(format!("%{}$@", position)),
                    FormatterMode::Plural => Ok(format!("%arg")),
                }
            }
            AstElement::Number { value, .. } => {
                if !self.argument_positions.contains(value) {
                    self.argument_positions.insert(value.to_string());
                }
                let index = self
                    .argument_positions
                    .iter()
                    .position(|x| x == value)
                    .ok_or_else(|| format!("Failed to find position for number '{}'", value))?;
                let position = index.checked_add(1)
                    .ok_or_else(|| format!("Position overflow for number '{}'", value))?;
                Ok(format!("%{}$lld", position))
            }
            AstElement::Date { value, .. } => {
                if !self.argument_positions.contains(value) {
                    self.argument_positions.insert(value.to_string());
                }
                let index = self
                    .argument_positions
                    .iter()
                    .position(|x| x == value)
                    .ok_or_else(|| format!("Failed to find position for date '{}'", value))?;
                let position = index.checked_add(1)
                    .ok_or_else(|| format!("Position overflow for date '{}'", value))?;
                Ok(format!("%{}$@", position))
            }
            AstElement::Plural { value, .. } => Ok(format!("%#@{}@", value)),
            AstElement::Select { value, .. } => Ok(format!("%#@{}@", value)),
            AstElement::Pound(_) => Ok("#".to_string()),
            _ => Ok("".to_string()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::models::LocalizableICUMessage;
    use icu_messageformat_parser::AstElement;

    #[test]
    fn test_format() {
        let mut formatter = super::XCStringFormatter::new(super::FormatterMode::StringUnit);
        let element = AstElement::Argument {
            value: "name1".to_string(),
            span: None,
        };
        assert_eq!(formatter.format(&element).unwrap(), "%1$@");
        let element = AstElement::Literal {
            value: "Hello, ".to_string(),
            span: None,
        };
        assert_eq!(formatter.format(&element).unwrap(), "Hello, ");
        let element = AstElement::Argument {
            value: "name2".to_string(),
            span: None,
        };
        assert_eq!(formatter.format(&element).unwrap(), "%2$@");
    }

    #[test]
    fn test_deserialize_localizable_icu_message() {
        let json = r#"{
        "key": "key",
        "messages": {
           "en": { "value": "Hello, {name1} and {name2}!", "state": "translated" },
           "es": { "value": "¡Hola, {name2} y {name1}!", "state": "translated" }
        }
     }"#;
        let message: LocalizableICUMessage = serde_json::from_str(json).unwrap();
        assert_eq!(message.key, "key");
        assert_eq!(message.messages.len(), 2);
        assert_eq!(
            message.messages.get("en").unwrap().value,
            "Hello, {name1} and {name2}!"
        );
        assert_eq!(
            message.messages.get("es").unwrap().value,
            "¡Hola, {name2} y {name1}!"
        );
    }
}
