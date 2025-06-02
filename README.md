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

## ⚠️ Important Limitations and Edge Cases

### Special Characters and Escaping

ICU MessageFormat requires special handling for certain characters:

**Problem Characters:**
- Empty curly braces `{}` are interpreted as empty arguments (invalid)
- Unescaped curly braces in literal text cause parsing errors

**Solution - Use ICU Escaping:**
```json
// ❌ Wrong - will cause parsing errors
{
  "value": "Special chars: {}"
}

// ✅ Correct - properly escaped
{
  "value": "Special chars: '{'}'}'"
}
```

**ICU Escaping Rules:**
- Literal curly braces: `'{'` and `'}'`
- Literal single quotes: `''`

### Variable Consistency Validation

This converter validates that all languages use the same variable names and counts:

```json
// ❌ Will fail validation
{
  "en": { "value": "Hello {name} and {age}!" },
  "ja": { "value": "こんにちは {name} さん!" }  // Missing {age}
}

// ✅ Correct - consistent variables
{
  "en": { "value": "Hello {name}!" },
  "ja": { "value": "こんにちは {name} さん!" }
}
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
| `--split-select-elements` | | Split select elements into separate keys | `true` | `true`, `false` |

#### Select Element Behavior

Apple's String Catalog format doesn't natively support ICU MessageFormat's select elements. This converter provides a `--split-select-elements` option to control how select elements are handled:

**When `--split-select-elements true` (default):**
- ✅ Select elements are automatically split into separate string keys
- Each select case becomes a separate key (e.g., `key_male`, `key_female`, `key_other`)
- Compatible with xcstrings format

**When `--split-select-elements false`:**
- ❌ Conversion fails with an error if select elements are found
- Useful for strict validation when select elements should not exist

**Example:**
```bash
# Enable select splitting (default)
cargo run -- -i input.json -o output.xcstrings -s en --split-select-elements true

# Disable select splitting (will error if select elements exist)
cargo run -- -i input.json -o output.xcstrings -s en --split-select-elements false
```

**Example Transformation:**
```
Input:  "{gender, select, male {He} female {She} other {They}} is online"
Output: Three separate keys:
- user_status_male: "He is online"
- user_status_female: "She is online" 
- user_status_other: "They is online"
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

### Variable Validation

This converter automatically validates that ICU MessageFormat variables are consistent across all languages for each key. This prevents runtime crashes and data inconsistencies.

**Validation Rules:**
- ✅ **Variable count must match** across all languages
- ✅ **Variable names must be identical** across all languages  
- ✅ **Variable types are preserved** (arguments, numbers, dates, plurals, selects)

**Example Error Cases:**

**Variable Count Mismatch:**
```json
{
  "key": "greeting",
  "messages": {
    "en": { "value": "Hello {name} and {age}!" },
    "ja": { "value": "こんにちは {name} さん！" }
  }
}
```
❌ Error: `Variable count mismatch in key 'greeting'. Language 'ja' has 1 variables, but expected 2`

**Variable Name Mismatch:**
```json
{
  "key": "greeting", 
  "messages": {
    "en": { "value": "Hello {firstName}!" },
    "ja": { "value": "こんにちは {lastName} さん！" }
  }
}
```
❌ Error: `Variable name mismatch in key 'greeting'. Language 'ja' contains variable 'lastName' which is not found in other languages. Expected variables: ["firstName"]`

**Correct Usage:**
```json
{
  "key": "greeting",
  "messages": {
    "en": { "value": "Hello {name} and {age}!" },
    "ja": { "value": "こんにちは {name} さん、{age} 歳ですね！" }
  }
}
```
✅ Success: Both languages use the same variables `{name}` and `{age}`

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## License

This project is licensed under the MIT License. See LICENSE file for details.
