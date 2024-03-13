use icu_messageformat_parser::AstElement;
use linked_hash_set::LinkedHashSet;

#[derive(Debug)]
pub struct XCStringFormatter {
    formatter_mode: FormatterMode,
    argument_positions: LinkedHashSet<String>,
}

#[derive(Debug)]
pub enum FormatterMode {
    StringUnit,
    Plural,
}

impl<'a> XCStringFormatter {
    pub fn new(mode: FormatterMode) -> Self {
        XCStringFormatter {
            formatter_mode: mode,
            argument_positions: LinkedHashSet::new(),
        }
    }

    pub fn format(&mut self, element: &'a AstElement) -> String {
        match &element {
            AstElement::Literal { value, span } => value.to_string(),
            AstElement::Argument { value, span } => {
                if !self.argument_positions.contains(value) {
                    self.argument_positions.insert(value.to_string());
                }
                let index = self
                    .argument_positions
                    .iter()
                    .position(|x| x.eq(value))
                    .unwrap();
                match self.formatter_mode {
                    FormatterMode::StringUnit => format!("%{}$@", index + 1),
                    FormatterMode::Plural => format!("%arg"),
                }
            }
            AstElement::Number { value, span, style } => {
                if !self.argument_positions.contains(value) {
                    self.argument_positions.insert(value.to_string());
                }
                let index = self
                    .argument_positions
                    .iter()
                    .position(|x| x == value)
                    .unwrap();
                format!("%{}$lld", index + 1)
            }
            AstElement::Plural {
                value,
                plural_type,
                span,
                offset,
                options,
            } => {
                format!("%#@{}@", value)
            }
            _ => "".to_string(),
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
        assert_eq!(formatter.format(&element), "%1$@");
        let element = AstElement::Literal {
            value: "Hello, ".to_string(),
            span: None,
        };
        assert_eq!(formatter.format(&element), "Hello, ");
        let element = AstElement::Argument {
            value: "name2".to_string(),
            span: None,
        };
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
        let message: LocalizableICUMessage = serde_json::from_str(json).unwrap();
        assert_eq!(message.key, "key");
        assert_eq!(message.messages.len(), 2);
        assert_eq!(
            message.messages.get("en").unwrap(),
            "Hello, {name1} and {name2}!"
        );
        assert_eq!(
            message.messages.get("es").unwrap(),
            "¡Hola, {name2} y {name1}!"
        );
    }
}
