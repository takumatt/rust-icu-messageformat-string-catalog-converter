use clap::Parser;
use linked_hash_map::LinkedHashMap;

mod models;
mod xcstring_converter;
mod xcstring_formatter;
mod xcstring_substitution_builder;
mod xcstrings;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // The path to the input file
    #[arg(short, long, value_name = "PATH")]
    input: String,

    // The path for the output file
    #[arg(short, long, value_name = "PATH")]
    output: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    _debug();
    Ok(())
}

fn _debug() {
    let message = models::LocalizableICUMessage::new(
        "key".to_string(),
        vec![
            (
                "en".to_string(),
                "Cart: {itemCount, plural, one {{itemCount} item} other {{itemCount} items}}"
                    .to_string(),
            ),
            ("es".to_string(), "¡Hola, {name2} y {name1}!".to_string()),
        ]
        .into_iter()
        .collect(),
    );
    let converter = xcstring_converter::XCStringConverter::new(
        "en".to_string(),
        models::ConverterOptions::default(),
        icu_messageformat_parser::ParserOptions::default(),
    );
    let xcstring = converter.convert(vec![message]);
    println!("{}", serde_json::to_string_pretty(&xcstring).unwrap());
}

impl models::LocalizableICUMessage {
    fn new(key: String, messages: LinkedHashMap<String, String>) -> models::LocalizableICUMessage {
        models::LocalizableICUMessage {
            key: key,
            messages: messages,
        }
    }
}
