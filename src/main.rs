mod xc_string_converter;
mod models;
mod xcstring;

fn main() {
    println!("Hello, world!");
    let message = models::LocalizableICUMessage::new("key".to_string(), vec![
        ("en".to_string(), "Hello, {name}!".to_string()),
        ("es".to_string(), "¡Hola, {name}!".to_string()),
    ].into_iter().collect());
    print!("{:?}", message.messages);
    let converter = xc_string_converter::XCStringConverter::new(icu_messageformat_parser::ParserOptions::default());
    converter.convert(message);
}

impl models::LocalizableICUMessage {
    fn new(key: String, messages: std::collections::HashMap<String, String>) -> Self {
        Self {
            key,
            messages: messages,
        }
    }
}
