use clap::Parser;
use std::fs;

mod models;
mod xcstring_converter;
mod xcstring_formatter;
mod xcstring_substitution_builder;
mod xcstrings;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path to the input file
    #[arg(short, long, value_name = "PATH")]
    input: String,

    /// The path for the output file
    #[arg(short, long, value_name = "PATH")]
    output: String,

    /// The source language code (e.g., "en", "ja")
    #[arg(short, long, value_name = "LANG")]
    source_language: String,

    /// The version of the xcstrings file (default: "1.0")
    #[arg(short, long, value_name = "VERSION", default_value = "1.0")]
    version: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Read input file
    let input_content = fs::read_to_string(&args.input)?;
    let messages: models::LocalizableICUStrings = serde_json::from_str(&input_content)?;

    // Convert to xcstrings format
    let converter = xcstring_converter::XCStringConverter::new(
        args.source_language,
        models::ConverterOptions::default(),
        icu_messageformat_parser::ParserOptions::default(),
    );
    let messages: Vec<models::LocalizableICUMessage> = messages.strings.into_iter().map(|s| s.into()).collect();
    let mut xcstrings = converter.convert(messages);
    xcstrings.version = args.version;

    // Write output file
    let output_content = serde_json::to_string_pretty(&xcstrings)?;
    fs::write(&args.output, output_content)?;

    Ok(())
}
