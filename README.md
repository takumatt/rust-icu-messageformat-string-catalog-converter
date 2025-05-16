# rust-icu-messageformat-string-catalog-converter

## Overview
This Rust library provides a convenient way to convert ICU Message Format strings into String Catalogs.

- Supported elements
  - [x] Literal
  - [x] Argument
  - [x] Number
  - [x] Plural (partial)

## Usage

### CLI Tool

```bash
# Convert ICU Message Format strings to String Catalog format
cargo run -- -i input.json -o output.xcstrings -s ja -v 1.0
```

#### Options

- `-i, --input <PATH>`: Path to the input JSON file
- `-o, --output <PATH>`: Path for the output xcstrings file
- `-s, --source-language <LANG>`: Source language code (e.g., "en", "ja")
- `-v, --version <VERSION>`: Version of the xcstrings file (default: "1.0")

#### Input Format

```json
{
  "strings": [
    {
      "key": "greeting",
      "messages": {
        "ja": {
          "value": "こんにちは {name} さん",
          "state": "translated"
        },
        "en": {
          "value": "Hello {name}",
          "state": "needs_review"
        }
      },
      "comment": "A greeting message with the user's name"
    }
  ]
}
```

#### Output Format

The tool converts the input into the String Catalog format (xcstrings):

```json
{
  "sourceLanguage": "ja",
  "strings": {
    "greeting": {
      "extractionState": "manual",
      "localizations": {
        "ja": {
          "stringUnit": {
            "state": "translated",
            "value": "こんにちは %1$@ さん"
          }
        },
        "en": {
          "stringUnit": {
            "state": "needs_review",
            "value": "Hello %1$@"
          }
        }
      }
    }
  },
  "version": "1.0"
}
```
