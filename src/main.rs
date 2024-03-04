mod xc_string_converter;
mod models;

fn main() {
    println!("Hello, world!");
    let message = models::LocalizableICUMessage::new("key".to_string(), vec![
        "Hello, {name}!".to_string(),
        "¡Hola, {name}!".to_string(),
    ]);
    print!("{:?}", message.messages);
    let converter = xc_string_converter::XCStringConverter::new(icu_messageformat_parser::ParserOptions::default());
    converter.convert(message);
}

impl models::LocalizableICUMessage {
    fn new(key: String, messages: Vec<String>) -> Self {
        let mut message_map = std::collections::HashMap::new();
        for message in messages {
            message_map.insert(message.clone(), message);
        }
        Self {
            key,
            messages: message_map,
        }
    }
}
