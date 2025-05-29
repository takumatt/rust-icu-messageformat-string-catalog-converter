#!/bin/bash

# Create output directory if it doesn't exist
mkdir -p samples/TestApp/TestApp

# Generate xcstrings for each test fixture
for fixture in tests/fixtures/*; do
    if [ -f "$fixture" ]; then
        fixture_name=$(basename "$fixture")
        echo "Generating xcstrings for ${fixture_name}..."
        # Extract the input section (everything before the first ---)
        input_section=$(sed -n '/^---/q;p' "$fixture")
        # Create a temporary file for the input
        temp_input=$(mktemp)
        echo "$input_section" > "$temp_input"
        # Run the converter and save the output
        cargo run -- --input "$temp_input" --output "samples/TestApp/TestApp/${fixture_name}.xcstrings" --source-language ja --version 1.0 --localization-state translated
        # Clean up the temporary file
        rm "$temp_input"
    fi
done

echo "Generated xcstrings files are in samples/TestApp/TestApp/" 