use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_icu_messageformat_string_catalog_converter::{
    converter::XCStringConverter,
    models::{LocalizableICUMessage, LocalizableICUMessageValue, LocalizableICUString, LocalizableICUStrings},
};
use linked_hash_map::LinkedHashMap;

fn generate_large_test_data(strings_count: usize) -> LocalizableICUStrings {
    let mut strings = Vec::with_capacity(strings_count);
    
    for i in 0..strings_count {
        let key = format!("key_{:06}", i);
        let mut messages = LinkedHashMap::new();
        
        // 英語のメッセージ
        messages.insert("en".to_string(), LocalizableICUMessageValue {
            value: format!("Hello, {{name_{}}}! You have {{count_{}}} messages.", i, i),
            state: "translated".to_string(),
        });
        
        // 日本語のメッセージ
        messages.insert("ja".to_string(), LocalizableICUMessageValue {
            value: format!("こんにちは、{{name_{}}}さん！{{count_{}}}件のメッセージがあります。", i, i),
            state: "translated".to_string(),
        });
        
        // 韓国語のメッセージ
        messages.insert("ko".to_string(), LocalizableICUMessageValue {
            value: format!("안녕하세요, {{name_{}}}님! {{count_{}}}개의 메시지가 있습니다.", i, i),
            state: "translated".to_string(),
        });
        
        strings.push(LocalizableICUString {
            key,
            messages,
            comment: Some(format!("Generated test message {}", i)),
        });
    }
    
    LocalizableICUStrings { strings }
}

fn generate_large_test_data_with_plurals(strings_count: usize) -> LocalizableICUStrings {
    let mut strings = Vec::with_capacity(strings_count);
    
    for i in 0..strings_count {
        let key = format!("plural_key_{:06}", i);
        let mut messages = LinkedHashMap::new();
        
        // 英語の複数形メッセージ
        messages.insert("en".to_string(), LocalizableICUMessageValue {
            value: format!("You have {{count_{}, plural, =0 {{no messages}} =1 {{one message}} other {{# messages}}}}.", i),
            state: "translated".to_string(),
        });
        
        // 日本語の複数形メッセージ
        messages.insert("ja".to_string(), LocalizableICUMessageValue {
            value: format!("{{count_{}, plural, =0 {{メッセージがありません}} =1 {{1件のメッセージがあります}} other {{#件のメッセージがあります}}}}。", i),
            state: "translated".to_string(),
        });
        
        strings.push(LocalizableICUString {
            key,
            messages,
            comment: Some(format!("Generated plural test message {}", i)),
        });
    }
    
    LocalizableICUStrings { strings }
}

fn generate_large_test_data_with_selects(strings_count: usize) -> LocalizableICUStrings {
    let mut strings = Vec::with_capacity(strings_count);
    
    for i in 0..strings_count {
        let key = format!("select_key_{:06}", i);
        let mut messages = LinkedHashMap::new();
        
        // 英語のselectメッセージ
        messages.insert("en".to_string(), LocalizableICUMessageValue {
            value: format!("{{gender_{}, select, male {{He}} female {{She}} other {{They}}}} has {{count_{}}} messages.", i, i),
            state: "translated".to_string(),
        });
        
        // 日本語のselectメッセージ
        messages.insert("ja".to_string(), LocalizableICUMessageValue {
            value: format!("{{gender_{}, select, male {{彼}} female {{彼女}} other {{彼ら}}}}は{{count_{}}}件のメッセージを持っています。", i, i),
            state: "translated".to_string(),
        });
        
        strings.push(LocalizableICUString {
            key,
            messages,
            comment: Some(format!("Generated select test message {}", i)),
        });
    }
    
    LocalizableICUStrings { strings }
}

fn benchmark_convert_1000_strings(c: &mut Criterion) {
    let test_data = generate_large_test_data(1000);
    let converter = XCStringConverter::new(
        "en".to_string(),
        rust_icu_messageformat_string_catalog_converter::models::ConverterOptions::default(),
        icu_messageformat_parser::ParserOptions::default(),
    );
    
    c.bench_function("convert_1000_strings", |b| {
        b.iter(|| {
            let messages: Vec<LocalizableICUMessage> = test_data.strings.clone().into_iter().map(|s| s.into()).collect();
            let _result = converter.convert(black_box(messages));
        });
    });
}

fn benchmark_convert_5000_strings(c: &mut Criterion) {
    let test_data = generate_large_test_data(5000);
    let converter = XCStringConverter::new(
        "en".to_string(),
        rust_icu_messageformat_string_catalog_converter::models::ConverterOptions::default(),
        icu_messageformat_parser::ParserOptions::default(),
    );
    
    c.bench_function("convert_5000_strings", |b| {
        b.iter(|| {
            let messages: Vec<LocalizableICUMessage> = test_data.strings.clone().into_iter().map(|s| s.into()).collect();
            let _result = converter.convert(black_box(messages));
        });
    });
}

fn benchmark_convert_10000_strings(c: &mut Criterion) {
    let test_data = generate_large_test_data(10000);
    let converter = XCStringConverter::new(
        "en".to_string(),
        rust_icu_messageformat_string_catalog_converter::models::ConverterOptions::default(),
        icu_messageformat_parser::ParserOptions::default(),
    );
    
    c.bench_function("convert_10000_strings", |b| {
        b.iter(|| {
            let messages: Vec<LocalizableICUMessage> = test_data.strings.clone().into_iter().map(|s| s.into()).collect();
            let _result = converter.convert(black_box(messages));
        });
    });
}

fn benchmark_convert_10000_strings_parallel(c: &mut Criterion) {
    let test_data = generate_large_test_data(10000);
    let converter = XCStringConverter::new(
        "en".to_string(),
        rust_icu_messageformat_string_catalog_converter::models::ConverterOptions::default(),
        icu_messageformat_parser::ParserOptions::default(),
    );
    
    c.bench_function("convert_10000_strings_parallel", |b| {
        b.iter(|| {
            let messages: Vec<LocalizableICUMessage> = test_data.strings.clone().into_iter().map(|s| s.into()).collect();
            let _result = converter.convert_parallel(black_box(messages));
        });
    });
}

fn benchmark_convert_with_plurals(c: &mut Criterion) {
    let test_data = generate_large_test_data_with_plurals(1000);
    let converter = XCStringConverter::new(
        "en".to_string(),
        rust_icu_messageformat_string_catalog_converter::models::ConverterOptions::default(),
        icu_messageformat_parser::ParserOptions::default(),
    );
    
    c.bench_function("convert_1000_strings_with_plurals", |b| {
        b.iter(|| {
            let messages: Vec<LocalizableICUMessage> = test_data.strings.clone().into_iter().map(|s| s.into()).collect();
            let _result = converter.convert(black_box(messages));
        });
    });
}

fn benchmark_convert_with_selects(c: &mut Criterion) {
    let test_data = generate_large_test_data_with_selects(1000);
    let converter = XCStringConverter::new(
        "en".to_string(),
        rust_icu_messageformat_string_catalog_converter::models::ConverterOptions::default(),
        icu_messageformat_parser::ParserOptions::default(),
    );
    
    c.bench_function("convert_1000_strings_with_selects", |b| {
        b.iter(|| {
            let messages: Vec<LocalizableICUMessage> = test_data.strings.clone().into_iter().map(|s| s.into()).collect();
            let _result = converter.convert(black_box(messages));
        });
    });
}

fn benchmark_convert_mixed_content(c: &mut Criterion) {
    let mut test_data = generate_large_test_data(800);
    let mut plural_data = generate_large_test_data_with_plurals(100);
    let mut select_data = generate_large_test_data_with_selects(100);
    
    test_data.strings.append(&mut plural_data.strings);
    test_data.strings.append(&mut select_data.strings);
    
    let converter = XCStringConverter::new(
        "en".to_string(),
        rust_icu_messageformat_string_catalog_converter::models::ConverterOptions::default(),
        icu_messageformat_parser::ParserOptions::default(),
    );
    
    c.bench_function("convert_1000_strings_mixed_content", |b| {
        b.iter(|| {
            let messages: Vec<LocalizableICUMessage> = test_data.strings.clone().into_iter().map(|s| s.into()).collect();
            let _result = converter.convert(black_box(messages));
        });
    });
}

fn benchmark_memory_usage_analysis(c: &mut Criterion) {
    let test_data = generate_large_test_data(10000);
    let converter = XCStringConverter::new(
        "en".to_string(),
        rust_icu_messageformat_string_catalog_converter::models::ConverterOptions::default(),
        icu_messageformat_parser::ParserOptions::default(),
    );
    
    c.bench_function("memory_usage_10000_strings", |b| {
        b.iter(|| {
            let messages: Vec<LocalizableICUMessage> = test_data.strings.clone().into_iter().map(|s| s.into()).collect();
            let result = converter.convert(black_box(messages)).unwrap();
            // 結果のサイズを測定
            let _size = std::mem::size_of_val(&result);
        });
    });
}

fn benchmark_serialization_performance(c: &mut Criterion) {
    let test_data = generate_large_test_data(5000);
    let converter = XCStringConverter::new(
        "en".to_string(),
        rust_icu_messageformat_string_catalog_converter::models::ConverterOptions::default(),
        icu_messageformat_parser::ParserOptions::default(),
    );
    
    let messages: Vec<LocalizableICUMessage> = test_data.strings.into_iter().map(|s| s.into()).collect();
    let xcstrings = converter.convert(messages).unwrap();
    
    c.bench_function("serialize_5000_strings_to_json", |b| {
        b.iter(|| {
            let _json = serde_json::to_string_pretty(black_box(&xcstrings));
        });
    });
}

criterion_group!(
    benches,
    benchmark_convert_1000_strings,
    benchmark_convert_5000_strings,
    benchmark_convert_10000_strings,
    benchmark_convert_10000_strings_parallel,
    benchmark_convert_with_plurals,
    benchmark_convert_with_selects,
    benchmark_convert_mixed_content,
    benchmark_memory_usage_analysis,
    benchmark_serialization_performance
);
criterion_main!(benches); 