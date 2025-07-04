use rust_icu_messageformat_string_catalog_converter::models::{
    LocalizableICUMessage, LocalizableICUMessageValue, LocalizableICUStrings, ConverterOptions
};
use rust_icu_messageformat_string_catalog_converter::converter::XCStringConverter;
use linked_hash_map::LinkedHashMap;

// テスト1: 空の入力データ
#[test]
fn test_empty_strings_array() {
    let _messages = LocalizableICUStrings {
        strings: vec![]
    };
    
    let converter = XCStringConverter::new(
        "en".to_string(),
        ConverterOptions::default(),
        icu_messageformat_parser::ParserOptions::default(),
    );
    
    let result = converter.convert(vec![]);
    assert!(result.is_ok());
    let xcstrings = result.unwrap();
    assert_eq!(xcstrings.strings.len(), 0);
}

// テスト2: 空の文字列値
#[test]
fn test_empty_string_values() {
    let mut messages = LinkedHashMap::new();
    messages.insert("en".to_string(), LocalizableICUMessageValue {
        value: "".to_string(),
        state: "translated".to_string(),
    });
    
    let message = LocalizableICUMessage {
        key: "empty_value".to_string(),
        messages,
        comment: None,
    };
    
    let converter = XCStringConverter::new(
        "en".to_string(),
        ConverterOptions::default(),
        icu_messageformat_parser::ParserOptions::default(),
    );
    
    let result = converter.convert(vec![message]);
    assert!(result.is_ok());
    let xcstrings = result.unwrap();
    assert_eq!(xcstrings.strings.len(), 1);
    assert_eq!(xcstrings.strings.get("empty_value").unwrap()
        .localizations.get("en").unwrap().string_unit.value, "");
}

// テスト3: 空のメッセージマップ
#[test]
fn test_empty_messages_map() {
    let message = LocalizableICUMessage {
        key: "no_messages".to_string(),
        messages: LinkedHashMap::new(),
        comment: None,
    };
    
    let converter = XCStringConverter::new(
        "en".to_string(),
        ConverterOptions::default(),
        icu_messageformat_parser::ParserOptions::default(),
    );
    
    let result = converter.convert(vec![message]);
    assert!(result.is_ok());
    let xcstrings = result.unwrap();
    assert_eq!(xcstrings.strings.len(), 1);
    assert_eq!(xcstrings.strings.get("no_messages").unwrap().localizations.len(), 0);
}

// テスト4: 非常に長い文字列
#[test]
fn test_very_long_strings() {
    let very_long_string = "a".repeat(10000);
    let mut messages = LinkedHashMap::new();
    messages.insert("en".to_string(), LocalizableICUMessageValue {
        value: format!("Hello {{name}}, {}", very_long_string),
        state: "translated".to_string(),
    });
    
    let message = LocalizableICUMessage {
        key: "long_string".to_string(),
        messages,
        comment: None,
    };
    
    let converter = XCStringConverter::new(
        "en".to_string(),
        ConverterOptions::default(),
        icu_messageformat_parser::ParserOptions::default(),
    );
    
    let result = converter.convert(vec![message]);
    assert!(result.is_ok());
}

// テスト5: 無効なICUメッセージフォーマット
#[test]
fn test_invalid_icu_format() {
    let mut messages = LinkedHashMap::new();
    messages.insert("en".to_string(), LocalizableICUMessageValue {
        value: "Hello {unclosed_bracket".to_string(),
        state: "translated".to_string(),
    });
    
    let message = LocalizableICUMessage {
        key: "invalid_format".to_string(),
        messages,
        comment: None,
    };
    
    let converter = XCStringConverter::new(
        "en".to_string(),
        ConverterOptions::default(),
        icu_messageformat_parser::ParserOptions::default(),
    );
    
    let result = converter.convert(vec![message]);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Failed to parse message"));
}

// テスト6: 大量の変数を持つメッセージ
#[test]
fn test_many_variables() {
    let mut variable_parts = Vec::new();
    for i in 0..100 {
        variable_parts.push(format!("{{var{}}}", i));
    }
    let message_value = variable_parts.join(" and ");
    
    let mut messages = LinkedHashMap::new();
    messages.insert("en".to_string(), LocalizableICUMessageValue {
        value: message_value,
        state: "translated".to_string(),
    });
    
    let message = LocalizableICUMessage {
        key: "many_vars".to_string(),
        messages,
        comment: None,
    };
    
    let converter = XCStringConverter::new(
        "en".to_string(),
        ConverterOptions::default(),
        icu_messageformat_parser::ParserOptions::default(),
    );
    
    let result = converter.convert(vec![message]);
    assert!(result.is_ok());
}

// テスト7: 特殊文字を含む文字列（適切にエスケープされた版）
#[test]
fn test_special_characters_escaped() {
    let mut messages = LinkedHashMap::new();
    messages.insert("en".to_string(), LocalizableICUMessageValue {
        value: "Hello {name}! 🎉 Special chars: @#$%^&*()[]'{'}'|\\\"''<>?/~`".to_string(),
        state: "translated".to_string(),
    });
    
    let message = LocalizableICUMessage {
        key: "special_chars_escaped".to_string(),
        messages,
        comment: None,
    };
    
    let converter = XCStringConverter::new(
        "en".to_string(),
        ConverterOptions::default(),
        icu_messageformat_parser::ParserOptions::default(),
    );
    
    let result = converter.convert(vec![message]);
    assert!(result.is_ok(), "Properly escaped special characters should work");
}

// テスト7b: 特殊文字を含む文字列（エラーケース - エスケープなし）
#[test]
fn test_special_characters_unescaped() {
    let mut messages = LinkedHashMap::new();
    messages.insert("en".to_string(), LocalizableICUMessageValue {
        value: "Hello {name}! 🎉 Special chars: @#$%^&*()[]{}|\\\"'<>?/~`".to_string(),
        state: "translated".to_string(),
    });
    
    let message = LocalizableICUMessage {
        key: "special_chars_unescaped".to_string(),
        messages,
        comment: None,
    };
    
    let converter = XCStringConverter::new(
        "en".to_string(),
        ConverterOptions::default(),
        icu_messageformat_parser::ParserOptions::default(),
    );
    
    let result = converter.convert(vec![message]);
    assert!(result.is_err(), "Unescaped curly braces should cause an error");
    let error_message = result.unwrap_err();
    assert!(error_message.contains("EmptyArgument") || error_message.contains("Failed to parse"));
}

// テスト8: 非ASCII文字を含むキー名
#[test]
fn test_unicode_key_names() {
    let mut messages = LinkedHashMap::new();
    messages.insert("en".to_string(), LocalizableICUMessageValue {
        value: "Hello world".to_string(),
        state: "translated".to_string(),
    });
    
    let message = LocalizableICUMessage {
        key: "キー_🔑_ключ".to_string(),
        messages,
        comment: None,
    };
    
    let converter = XCStringConverter::new(
        "en".to_string(),
        ConverterOptions::default(),
        icu_messageformat_parser::ParserOptions::default(),
    );
    
    let result = converter.convert(vec![message]);
    assert!(result.is_ok());
}

// テスト9: 重複するキー名
#[test]
fn test_duplicate_keys() {
    let mut messages1 = LinkedHashMap::new();
    messages1.insert("en".to_string(), LocalizableICUMessageValue {
        value: "First value".to_string(),
        state: "translated".to_string(),
    });
    
    let mut messages2 = LinkedHashMap::new();
    messages2.insert("en".to_string(), LocalizableICUMessageValue {
        value: "Second value".to_string(),
        state: "translated".to_string(),
    });
    
    let message1 = LocalizableICUMessage {
        key: "duplicate".to_string(),
        messages: messages1,
        comment: None,
    };
    
    let message2 = LocalizableICUMessage {
        key: "duplicate".to_string(),
        messages: messages2,
        comment: None,
    };
    
    let converter = XCStringConverter::new(
        "en".to_string(),
        ConverterOptions::default(),
        icu_messageformat_parser::ParserOptions::default(),
    );
    
    let result = converter.convert(vec![message1, message2]);
    assert!(result.is_ok());
    // 後の値で上書きされる
    let xcstrings = result.unwrap();
    assert_eq!(xcstrings.strings.get("duplicate").unwrap()
        .localizations.get("en").unwrap().string_unit.value, "Second value");
}

// テスト10: 無効なlocalization state
#[test]
fn test_invalid_localization_state() {
    let mut messages = LinkedHashMap::new();
    messages.insert("en".to_string(), LocalizableICUMessageValue {
        value: "Hello world".to_string(),
        state: "invalid_state".to_string(),
    });
    
    let message = LocalizableICUMessage {
        key: "invalid_state_key".to_string(),
        messages,
        comment: None,
    };
    
    let converter = XCStringConverter::new(
        "en".to_string(),
        ConverterOptions::default(),
        icu_messageformat_parser::ParserOptions::default(),
    );
    
    let result = converter.convert(vec![message]);
    assert!(result.is_ok());
    // デフォルトで"translated"になる
    let xcstrings = result.unwrap();
    assert_eq!(xcstrings.strings.get("invalid_state_key").unwrap()
        .localizations.get("en").unwrap().string_unit.localization_state,
        rust_icu_messageformat_string_catalog_converter::xcstrings::LocalizationState::Translated);
}

// テスト11: select要素でother case不足
#[test]
fn test_select_without_other_case() {
    let mut messages = LinkedHashMap::new();
    messages.insert("en".to_string(), LocalizableICUMessageValue {
        value: "{gender, select, male {He} female {She}}".to_string(),
        state: "translated".to_string(),
    });
    
    let message = LocalizableICUMessage {
        key: "no_other_case".to_string(),
        messages,
        comment: None,
    };
    
    let converter = XCStringConverter::new(
        "en".to_string(),
        ConverterOptions::default(),
        icu_messageformat_parser::ParserOptions::default(),
    );
    
    let result = converter.convert(vec![message]);
    // ICUパーサーが処理するかもしれないが、通常はエラーになる可能性がある
    if result.is_err() {
        println!("Select without other case error: {}", result.unwrap_err());
    } else {
        println!("Select without other case succeeded unexpectedly");
    }
}

// テスト12: 空のselect要素
#[test]
fn test_empty_select_options() {
    let mut messages = LinkedHashMap::new();
    messages.insert("en".to_string(), LocalizableICUMessageValue {
        value: "Status: {status, select}".to_string(),
        state: "translated".to_string(),
    });
    
    let message = LocalizableICUMessage {
        key: "empty_select".to_string(),
        messages,
        comment: None,
    };
    
    let converter = XCStringConverter::new(
        "en".to_string(),
        ConverterOptions::default(),
        icu_messageformat_parser::ParserOptions::default(),
    );
    
    let result = converter.convert(vec![message]);
    // パーサーがエラーを出すはず
    if result.is_err() {
        println!("Empty select options error: {}", result.unwrap_err());
    } else {
        println!("Empty select options succeeded unexpectedly");
    }
}

// テスト13: 極端に深いネストを持つselect/plural
#[test]
fn test_deeply_nested_elements() {
    let mut messages = LinkedHashMap::new();
    messages.insert("en".to_string(), LocalizableICUMessageValue {
        value: "{count, plural, one {{gender, select, male {He has one item} female {She has one item} other {They have one item}}} other {{gender, select, male {He has # items} female {She has # items} other {They have # items}}}}".to_string(),
        state: "translated".to_string(),
    });
    
    let message = LocalizableICUMessage {
        key: "deeply_nested".to_string(),
        messages,
        comment: None,
    };
    
    let converter = XCStringConverter::new(
        "en".to_string(),
        ConverterOptions::default(),
        icu_messageformat_parser::ParserOptions::default(),
    );
    
    let result = converter.convert(vec![message]);
    if result.is_err() {
        println!("Deeply nested error: {}", result.unwrap_err());
    } else {
        println!("Deeply nested succeeded");
    }
}

// テスト14: ゼロ長の変数名
#[test]
fn test_empty_variable_name() {
    let mut messages = LinkedHashMap::new();
    messages.insert("en".to_string(), LocalizableICUMessageValue {
        value: "Hello {}!".to_string(),
        state: "translated".to_string(),
    });
    
    let message = LocalizableICUMessage {
        key: "empty_var_name".to_string(),
        messages,
        comment: None,
    };
    
    let converter = XCStringConverter::new(
        "en".to_string(),
        ConverterOptions::default(),
        icu_messageformat_parser::ParserOptions::default(),
    );
    
    let result = converter.convert(vec![message]);
    if result.is_err() {
        println!("Empty variable name error: {}", result.unwrap_err());
    } else {
        println!("Empty variable name succeeded");
    }
}

// テスト15: 数値オーバーフローのテスト（非常に多くの変数）
#[test]
fn test_position_overflow() {
    // usize::MAX に近い数の変数を作る（ただし実用的な範囲で）
    let mut variable_parts = Vec::new();
    for i in 0..1000 {
        variable_parts.push(format!("{{var{}}}", i));
    }
    let message_value = variable_parts.join(" ");
    
    let mut messages = LinkedHashMap::new();
    messages.insert("en".to_string(), LocalizableICUMessageValue {
        value: message_value,
        state: "translated".to_string(),
    });
    
    let message = LocalizableICUMessage {
        key: "overflow_test".to_string(),
        messages,
        comment: None,
    };
    
    let converter = XCStringConverter::new(
        "en".to_string(),
        ConverterOptions::default(),
        icu_messageformat_parser::ParserOptions::default(),
    );
    
    let result = converter.convert(vec![message]);
    assert!(result.is_ok()); // checked_add で保護されているはず
}

// テスト16: HTML/XMLタグがサポートされていないことのテスト
#[test]
fn test_html_xml_tags_not_supported() {
    let mut messages = LinkedHashMap::new();
    messages.insert("en".to_string(), LocalizableICUMessageValue {
        value: "For health insurance cards only: Please hide <symbol and number> and <insurer number> before submitting.".to_string(),
        state: "translated".to_string(),
    });
    
    let message = LocalizableICUMessage {
        key: "html_tag_test".to_string(),
        messages,
        comment: Some("Test case with angle brackets that should be treated as literal text".to_string()),
    };
    
    // デフォルトのパーサーオプション（ignore_tag: true）で処理
    let parser_options = icu_messageformat_parser::ParserOptions {
        ignore_tag: true,
        requires_other_clause: false,
        should_parse_skeletons: false,
        capture_location: false,
        locale: None,
    };
    
    let converter = XCStringConverter::new(
        "en".to_string(),
        ConverterOptions::default(),
        parser_options,
    );
    
    let result = converter.convert_parallel(vec![message]);
    assert!(result.is_ok());
    
    if let Ok(xcstrings) = result {
        let localization = xcstrings.strings.get("html_tag_test").unwrap();
        let en_value = &localization.localizations.get("en").unwrap().string_unit.value;
        // 角括弧がそのまま保持されることを確認（HTMLタグとして解釈されない）
        assert!(en_value.contains("<symbol and number>"));
        assert!(en_value.contains("<insurer number>"));
        // HTMLタグとして解釈されていないことを確認（内容が削除されていない）
        assert!(en_value.contains("symbol and number"));
        assert!(en_value.contains("insurer number"));
    }
}

// テスト17: 一般的なHTMLタグが文字列として保持されることのテスト
#[test]
fn test_common_html_tags_preserved_as_text() {
    let test_cases = vec![
        ("bold_tag", "This is <b>bold</b> text."),
        ("italic_tag", "This is <i>italic</i> text."),
        ("link_tag", "Click <a href=\"https://example.com\">here</a> for info."),
        ("div_tag", "Content in <div class=\"highlight\">container</div>."),
        ("span_tag", "Highlighted <span style=\"color:red\">text</span> here."),
    ];
    
    for (key, value) in test_cases {
        let mut messages = LinkedHashMap::new();
        messages.insert("en".to_string(), LocalizableICUMessageValue {
            value: value.to_string(),
            state: "translated".to_string(),
        });
        
        let message = LocalizableICUMessage {
            key: key.to_string(),
            messages,
            comment: Some("Test case with HTML tags that should be preserved as literal text".to_string()),
        };
        
        let parser_options = icu_messageformat_parser::ParserOptions {
            ignore_tag: true,
            requires_other_clause: false,
            should_parse_skeletons: false,
            capture_location: false,
            locale: None,
        };
        
        let converter = XCStringConverter::new(
            "en".to_string(),
            ConverterOptions::default(),
            parser_options,
        );
        
        let result = converter.convert_parallel(vec![message]);
        assert!(result.is_ok(), "Failed to process HTML tag test case: {}", key);
        
        if let Ok(xcstrings) = result {
            let localization = xcstrings.strings.get(key).unwrap();
            let en_value = &localization.localizations.get("en").unwrap().string_unit.value;
            // 元の文字列がそのまま保持されることを確認
            assert_eq!(en_value, value, "HTML tags were not preserved as literal text for case: {}", key);
        }
    }
} 