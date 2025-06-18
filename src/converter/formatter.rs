use icu_messageformat_parser::{self, AstElement};
use std::collections::HashMap;

#[derive(Debug)]
pub enum FormatterMode {
    StringUnit,
    Plural,
}

pub struct XCStringFormatter {
    formatter_mode: FormatterMode,
    argument_positions: HashMap<String, usize>,
    next_position: usize,
}

impl XCStringFormatter {
    pub fn new(mode: FormatterMode) -> Self {
        XCStringFormatter {
            formatter_mode: mode,
            argument_positions: HashMap::new(),
            next_position: 1,
        }
    }

    pub fn format(&mut self, element: &AstElement) -> Result<String, String> {
        match &element {
            AstElement::Literal { value, .. } => Ok(value.clone()),
            AstElement::Argument { value, .. } => {
                let position = self.get_or_insert_position(value)?;
                match self.formatter_mode {
                    FormatterMode::StringUnit => Ok(format!("%{}$@", position)),
                    FormatterMode::Plural => Ok("%arg".to_string()),
                }
            }
            AstElement::Number { value, .. } => {
                let position = self.get_or_insert_position(value)?;
                Ok(format!("%{}$lld", position))
            }
            AstElement::Date { value, .. } => {
                let position = self.get_or_insert_position(value)?;
                Ok(format!("%{}$@", position))
            }
            AstElement::Plural { value, .. } => Ok(format!("%#@{}@", value)),
            AstElement::Select { value, .. } => Ok(format!("%#@{}@", value)),
            AstElement::Pound(_) => Ok("#".to_string()),
            _ => Ok(String::new()),
        }
    }

    fn get_or_insert_position(&mut self, value: &str) -> Result<usize, String> {
        if let Some(&position) = self.argument_positions.get(value) {
            Ok(position)
        } else {
            let position = self.next_position;
            self.argument_positions.insert(value.to_string(), position);
            self.next_position = self.next_position.checked_add(1)
                .ok_or_else(|| format!("Position overflow for argument '{}'", value))?;
            Ok(position)
        }
    }

    #[allow(dead_code)]
    pub fn format_batch(&mut self, elements: &[AstElement]) -> Result<String, String> {
        let estimated_capacity = elements.iter().fold(0, |acc, element| {
            acc + match element {
                AstElement::Literal { value, .. } => value.len(),
                AstElement::Argument { .. } => 4,
                AstElement::Number { .. } => 6,
                AstElement::Date { .. } => 4,
                AstElement::Plural { value, .. } => 3 + value.len(),
                AstElement::Select { value, .. } => 3 + value.len(),
                AstElement::Pound(_) => 1,
                _ => 0,
            }
        });
        
        let mut result = String::with_capacity(estimated_capacity);
        
        for element in elements {
            result.push_str(&self.format(element)?);
        }
        
        Ok(result)
    }

    #[allow(dead_code)]
    pub fn format_with_capacity(&mut self, element: &AstElement, capacity: usize) -> Result<String, String> {
        let mut result = String::with_capacity(capacity);
        result.push_str(&self.format(element)?);
        Ok(result)
    }

    /// より効率的なバッチ処理（事前に容量を計算）
    #[allow(dead_code)]
    pub fn format_batch_optimized(&mut self, elements: &[AstElement]) -> Result<String, String> {
        if elements.is_empty() {
            return Ok(String::new());
        }
        
        // より正確な容量推定
        let estimated_capacity = elements.iter().fold(0, |acc, element| {
            acc + match element {
                AstElement::Literal { value, .. } => value.len(),
                AstElement::Argument { .. } => 4, // "%1$@" の長さ
                AstElement::Number { .. } => 6,   // "%1$lld" の長さ
                AstElement::Date { .. } => 4,     // "%1$@" の長さ
                AstElement::Plural { value, .. } => 3 + value.len(), // "%#@...@" の長さ
                AstElement::Select { value, .. } => 3 + value.len(), // "%#@...@" の長さ
                AstElement::Pound(_) => 1,        // "#" の長さ
                _ => 0,
            }
        });
        
        let mut result = String::with_capacity(estimated_capacity);
        
        // 一度に全ての要素を処理
        for element in elements {
            match element {
                AstElement::Literal { value, .. } => result.push_str(value),
                AstElement::Argument { value, .. } => {
                    let position = self.get_or_insert_position(value)?;
                    match self.formatter_mode {
                        FormatterMode::StringUnit => result.push_str(&format!("%{}$@", position)),
                        FormatterMode::Plural => result.push_str("%arg"),
                    }
                }
                AstElement::Number { value, .. } => {
                    let position = self.get_or_insert_position(value)?;
                    result.push_str(&format!("%{}$lld", position));
                }
                AstElement::Date { value, .. } => {
                    let position = self.get_or_insert_position(value)?;
                    result.push_str(&format!("%{}$@", position));
                }
                AstElement::Plural { value, .. } => {
                    result.push_str(&format!("%#@{}@", value));
                }
                AstElement::Select { value, .. } => {
                    result.push_str(&format!("%#@{}@", value));
                }
                AstElement::Pound(_) => result.push('#'),
                _ => {}
            }
        }
        
        Ok(result)
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
    fn test_format_batch() {
        let mut formatter = super::XCStringFormatter::new(super::FormatterMode::StringUnit);
        let elements = vec![
            AstElement::Literal {
                value: "Hello, ".to_string(),
                span: None,
            },
            AstElement::Argument {
                value: "name1".to_string(),
                span: None,
            },
            AstElement::Literal {
                value: "!".to_string(),
                span: None,
            },
        ];
        assert_eq!(formatter.format_batch(&elements).unwrap(), "Hello, %1$@!");
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
