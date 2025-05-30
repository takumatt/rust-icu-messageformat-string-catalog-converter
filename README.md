# ICU MessageFormat to String Catalog Converter

A Rust library and CLI tool that converts ICU MessageFormat strings into Apple's String Catalog format (xcstrings).

## Features

### Supported ICU MessageFormat Elements

- ✅ **Literal text** - Plain text content
- ✅ **Arguments** - Variable placeholders (`{name}`)
- ✅ **Number formatting** - Number placeholders with formatting
- ✅ **Plural forms** - Pluralization rules (partial support)
- ✅ **Select elements** - Gender/context selection with automatic splitting

### Select Element Support

Apple's String Catalog format doesn't natively support ICU MessageFormat's select elements. This converter automatically splits select elements into separate string keys, making them compatible with xcstrings.

**Example:**
```
Input:  "{gender, select, male {He} female {She} other {They}} is online"
Output: Three separate keys:
- user_status_male: "He is online"
- user_status_female: "She is online" 
- user_status_other: "They is online"
```

## Installation

```bash
git clone https://github.com/your-repo/rust-icu-messageformat-string-catalog-converter
cd rust-icu-messageformat-string-catalog-converter
cargo build --release
```

## Usage

### CLI Tool

```bash
cargo run -- --input input.json --output output.xcstrings --source-language ja
```

#### Command Line Options

| Option | Short | Description | Default | Example |
|--------|-------|-------------|---------|---------|
| `--input` | `-i` | Path to input JSON file | Required | `input.json` |
| `--output` | `-o` | Path for output xcstrings file | Required | `output.xcstrings` |
| `--source-language` | `-s` | Source language code | Required | `en`, `ja`, `ko` |
| `--version` | `-v` | xcstrings file version | `"1.0"` | `"1.0"` |
| `--localization-state` | `-l` | Default localization state | `translated` | `translated`, `needs_review` |

#### Select Element Behavior

The `split_select_elements` option is **not available as a CLI flag**. It's currently hardcoded to `true` in the CLI tool, meaning:

- **Select elements are always split** into separate string keys when using the CLI
- **If you need to disable select splitting**, you must use the library programmatically in your own Rust code

**CLI Behavior:**
- ✅ Select elements are automatically split into separate keys (e.g., `key_male`, `key_female`, `key_other`)
- ❌ Cannot disable select splitting via command line

**Programmatic Usage:**
If you need to control select splitting behavior, use the library directly:

```rust
let mut options = ConverterOptions::default();
options.split_select_elements = false; // Disable splitting

let converter = XCStringConverter::new(source_language, options, parser_options);
```

### Input Format

The input JSON should follow this structure:

```json
{
  "strings": [
    {
      "key": "greeting",
      "messages": {
        "en": {
          "value": "Hello {name}",
          "state": "translated"
        },
        "ja": {
          "value": "こんにちは {name} さん",
          "state": "translated"
        },
        "ko": {
          "value": "안녕하세요 {name}",
          "state": "needs_review"
        }
      },
      "comment": "A greeting message with the user's name"
    },
    {
      "key": "user_status",
      "messages": {
        "en": {
          "value": "{gender, select, male {He} female {She} other {They}} is online.",
          "state": "translated"
        },
        "ja": {
          "value": "{gender, select, male {彼} female {彼女} other {その人}} は現在オンラインです。",
          "state": "translated"
        }
      },
      "comment": "User online status with gender selection"
    }
  ]
}
```

### Output Format

The tool generates Apple String Catalog format (xcstrings):

```json
{
  "sourceLanguage": "ja",
  "strings": {
    "greeting": {
      "extractionState": "manual",
      "localizations": {
        "en": {
          "stringUnit": {
            "state": "translated",
            "value": "Hello %1$@"
          }
        },
        "ja": {
          "stringUnit": {
            "state": "translated",
            "value": "こんにちは %1$@ さん"
          }
        },
        "ko": {
          "stringUnit": {
            "state": "needs_review",
            "value": "안녕하세요 %1$@"
          }
        }
      }
    },
    "user_status_male": {
      "extractionState": "manual",
      "localizations": {
        "en": {
          "stringUnit": {
            "state": "translated",
            "value": "He is online."
          }
        },
        "ja": {
          "stringUnit": {
            "state": "translated",
            "value": "彼 は現在オンラインです。"
          }
        }
      }
    },
    "user_status_female": {
      "extractionState": "manual",
      "localizations": {
        "en": {
          "stringUnit": {
            "state": "translated",
            "value": "She is online."
          }
        },
        "ja": {
          "stringUnit": {
            "state": "translated",
            "value": "彼女 は現在オンラインです。"
          }
        }
      }
    },
    "user_status_other": {
      "extractionState": "manual",
      "localizations": {
        "en": {
          "stringUnit": {
            "state": "translated",
            "value": "They is online."
          }
        },
        "ja": {
          "stringUnit": {
            "state": "translated",
            "value": "その人 は現在オンラインです。"
          }
        }
      }
    }
  },
  "version": "1.0"
}
```

## Sample Application

A complete iOS sample application is provided in `samples/TestApp/` demonstrating how to use the generated xcstrings files in SwiftUI.

### Using xcstrings in SwiftUI

```swift
import SwiftUI

struct ContentView: View {
    var body: some View {
        VStack {
            // Basic localization
            Text(String(localized: "greeting"))
            
            // With arguments
            Text(String(format: String(localized: "hello", table: "simple_argument"), "Alice", "Bob"))
            
            // Split select elements usage
            Text(String(localized: "user_status_male"))   // "He is online."
            Text(String(localized: "user_status_female")) // "She is online."
            Text(String(localized: "user_status_other"))  // "They is online."
        }
    }
}
```

### Testing Localization

1. Open the sample app in Xcode
2. Change device/simulator language in Settings
3. Run the app to see localized strings
4. Test different languages (English, Japanese, Korean)

## Development

### Running Tests

```bash
cargo test
```

### Generating Sample xcstrings

```bash
cd samples
./generate_xcstrings.sh
```

This generates xcstrings files for all test fixtures in `samples/TestApp/TestApp/`.

## Error Handling

### Select Element Errors

If `split_select_elements` is disabled and select elements are found:

```
Error: Select elements are not supported by xcstrings. Found in key: 'user_status'. Consider enabling split_select_elements option.
```

### Common Issues

- **Missing source language**: Ensure the source language exists in all message entries
- **Invalid JSON format**: Validate input JSON structure
- **Unsupported ICU elements**: Some advanced ICU features may not be fully supported

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## License

This project is licensed under the MIT License. See LICENSE file for details.
