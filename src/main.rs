mod xc_string_converter;
mod models;
mod xcstring;

fn main() {
    // TODO: Make source language always be the first language in the list
    let message = models::LocalizableICUMessage::new("key".to_string(), vec![
        ("en".to_string(), "Hello, {name1} and {name2}!".to_string()),
        ("es".to_string(), "¡Hola, {name2} y {name1}!".to_string()),
    ].into_iter().collect());
    let converter = xc_string_converter::XCStringConverter::new(
        "en".to_string(),
        icu_messageformat_parser::ParserOptions::default()
    );
    let xcstring = converter.convert(message);
    println!("{}", serde_json::to_string_pretty(&xcstring).unwrap());
}

impl models::LocalizableICUMessage {
    fn new(key: String, messages: Vec<(String, String)>) -> Self {
        Self {
            key,
            messages: messages,
        }
    }
}
