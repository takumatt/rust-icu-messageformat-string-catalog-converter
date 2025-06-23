use clap::Parser;
use std::fs;

mod models;
mod converter;
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

    /// The version of the generated xcstrings file (default: "1.0")
    #[arg(long = "xcstrings-version", value_name = "VERSION", default_value = "1.0")]
    xcstrings_version: String,

    /// The localization state for all strings (translated or needs_review)
    #[arg(short, long, value_name = "STATE", default_value = "translated")]
    localization_state: String,

    /// Split select elements into separate keys (default: true)
    #[arg(long, action = clap::ArgAction::Set, default_value = "true")]
    split_select_elements: bool,

    /// Ignore HTML/XML tags and treat them as literal text (default: true)
    #[arg(long, action = clap::ArgAction::Set, default_value = "true")]
    ignore_tag: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Read input file
    let input_content = fs::read_to_string(&args.input)?;
    let messages: models::LocalizableICUStrings = serde_json::from_str(&input_content)?;

    // Convert to xcstrings format
    let mut options = models::ConverterOptions::default();
    options.localization_state = match args.localization_state.as_str() {
        "translated" => xcstrings::LocalizationState::Translated,
        "needs_review" => xcstrings::LocalizationState::NeedsReview,
        _ => return Err("Invalid localization state. Must be 'translated' or 'needs_review'".into()),
    };
    options.split_select_elements = args.split_select_elements;
    
    // Create parser options with configurable ignore_tag to prevent HTML tag parsing
    let parser_options = icu_messageformat_parser::ParserOptions {
        ignore_tag: args.ignore_tag,
        requires_other_clause: false,
        should_parse_skeletons: false,
        capture_location: false,
        locale: None,
    };
    
    let converter = converter::XCStringConverter::new(
        args.source_language,
        options,
        parser_options,
    );
    let messages: Vec<models::LocalizableICUMessage> = messages.strings.into_iter().map(|s| s.into()).collect();
    
    // 並列処理版を使用（デフォルト）
    let mut xcstrings = converter.convert_parallel(messages)?;
    xcstrings.version = args.xcstrings_version;

    // Write output file
    let output_content = serde_json::to_string_pretty(&xcstrings)?;
    fs::write(&args.output, output_content)?;

    Ok(())
}
