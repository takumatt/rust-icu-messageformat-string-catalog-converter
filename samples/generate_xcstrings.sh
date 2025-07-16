#!/bin/bash

# Create output directory if it doesn't exist
mkdir -p samples/TestApp/TestApp

# Generate xcstrings for each test fixture
for fixture_dir in tests/fixtures/*/; do
    if [ -d "$fixture_dir" ]; then
        fixture_name=$(basename "$fixture_dir")
        input_file="${fixture_dir}input.json"
        if [ -f "$input_file" ]; then
            echo "Generating xcstrings for ${fixture_name}..."
            # Run the converter and save the output
            cargo run -- --input "$input_file" --output "samples/TestApp/TestApp/${fixture_name}.xcstrings" --source-language ja --xcstrings-version 1.0 --localization-state translated
        else
            echo "Warning: No input.json found in $fixture_dir"
        fi
    fi
done

echo "Generated xcstrings files are in samples/TestApp/TestApp/" 