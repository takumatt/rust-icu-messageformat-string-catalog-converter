# Sample xcstrings Files

This directory contains sample xcstrings files generated from our test fixtures. These files can be used to test the compatibility of the generated xcstrings with Xcode and Apple's localization system.

## Generated Files

The following xcstrings files are generated from our test fixtures:

- `simple_argument.xcstrings`: Basic string with a single argument
- `pluralization.xcstrings`: Strings with pluralization rules
- `multiple_languages.xcstrings`: Strings in multiple languages
- `icu_messageformat.xcstrings`: Complex ICU message format examples
- `multiple_arguments.xcstrings`: Strings with multiple arguments

## How to Use

1. Generate the xcstrings files:
   ```bash
   ./generate_xcstrings.sh
   ```

2. The files will be generated in the `TestApp/TestApp` directory

3. You can use these files in your Xcode project by:
   - Dragging them into your Xcode project
   - Adding them to your target
   - Using them in your SwiftUI views with `LocalizedStringKey`

## Testing in Xcode

To test these files in Xcode:

1. Create a new SwiftUI project
2. Add the generated xcstrings files to your project
3. Use the strings in your views:
   ```swift
   Text("greeting")
   Text("apple_count", count: 5)
   ```

## Notes

- These files are generated from our test fixtures and demonstrate various features of the string catalog format
- They can be used to verify that the generated xcstrings files are compatible with Xcode's localization system
- The files include examples of:
  - Basic string localization
  - Pluralization
  - Multiple languages
  - ICU message format features
  - Multiple arguments 