use crate::{
    xcstring_formatter::{FormatterMode, XCStringFormatter},
    xcstrings::{self, StringUnit, Substitution, VariationType, VariationValue},
};
use icu_messageformat_parser::{self, AstElement};
use linked_hash_map::LinkedHashMap;

pub struct XCStringSubstitutionBuilder {}

impl XCStringSubstitutionBuilder {
    pub fn new() -> XCStringSubstitutionBuilder {
        XCStringSubstitutionBuilder {}
    }

    pub fn build(&self, elements: Vec<AstElement>) -> Result<LinkedHashMap<String, Substitution>, String> {
        let mut formatter = XCStringFormatter::new(FormatterMode::Plural);
        let mut result = LinkedHashMap::new();
        
        for (index, element) in elements.iter().enumerate() {
            match element {
                AstElement::Plural {
                    value,
                    plural_type: _,
                    span: _,
                    offset: _,
                    options,
                } => {
                    let mut plural_map = LinkedHashMap::new();
                    for (key, value) in &options.0 {
                        let key_format = KeyFormat::from_string(&key.to_string())?;
                        let formatted_results: Result<Vec<String>, String> = value
                            .value
                            .iter()
                            .map(|element| formatter.format(element))
                            .collect();
                        let formatted_strings = formatted_results?
                            .join("");
                        plural_map.insert(
                            key_format.to_string(),
                            VariationValue {
                                string_unit: StringUnit {
                                    localization_state: xcstrings::LocalizationState::Translated,
                                    value: formatted_strings,
                                },
                            },
                        );
                    }
                    
                    let arg_num = index.checked_add(1)
                        .ok_or_else(|| format!("Argument number overflow for plural '{}'", value))?;
                    
                    result.insert(
                        value.clone(),
                        Substitution {
                            arg_num,
                            format_specifier: "lld".to_string(),
                            variations: VariationType::Plural(plural_map),
                        },
                    );
                }
                AstElement::Select {
                    value,
                    options,
                    ..
                } => {
                    let mut select_map = LinkedHashMap::new();
                    for (key, value) in &options.0 {
                        let formatted_results: Result<Vec<String>, String> = value
                            .value
                            .iter()
                            .map(|element| formatter.format(element))
                            .collect();
                        let formatted_strings = formatted_results?
                            .join("");
                        select_map.insert(
                            key.to_string(),
                            VariationValue {
                                string_unit: StringUnit {
                                    localization_state: xcstrings::LocalizationState::Translated,
                                    value: formatted_strings,
                                },
                            },
                        );
                    }
                    
                    let arg_num = index.checked_add(1)
                        .ok_or_else(|| format!("Argument number overflow for select '{}'", value))?;
                    
                    result.insert(
                        value.clone(),
                        Substitution {
                            arg_num,
                            format_specifier: "@".to_string(),
                            variations: VariationType::Select(select_map),
                        },
                    );
                }
                _ => {}
            }
        }
        
        Ok(result)
    }
}

enum KeyFormat {
    Zero,
    One,
    Other,
}

impl KeyFormat {
    fn from_string(key: &String) -> Result<KeyFormat, String> {
        match key.as_str() {
            "zero" => Ok(KeyFormat::Zero),
            "one" => Ok(KeyFormat::One),
            "two" => Err("String Catalog doesn't support custom keys starting with 'two'".to_string()),
            "few" => Err("String Catalog doesn't support custom keys starting with 'few'".to_string()),
            "many" => Err("String Catalog doesn't support custom keys starting with 'many'".to_string()),
            "other" => Ok(KeyFormat::Other),
            "=0" => Ok(KeyFormat::Zero), // "zero" is an alias for "=0"
            "=1" => Ok(KeyFormat::One),  // "one" is an alias for "=1"
            _ => {
                if key.as_str().starts_with("=") {
                    Err("String Catalog doesn't support custom keys starting with '=' except for zero and one".to_string())
                } else {
                    Err(format!("Unexpected key format: '{}'", key))
                }
            }
        }
    }

    fn to_string(&self) -> String {
        match self {
            KeyFormat::Zero => "zero".to_string(),
            KeyFormat::One => "one".to_string(),
            KeyFormat::Other => "other".to_string(),
        }
    }
}
