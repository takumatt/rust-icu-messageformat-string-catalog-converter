use criterion::{black_box, criterion_group, criterion_main, Criterion};
use icu_messageformat_parser::AstElement;
use rust_icu_messageformat_string_catalog_converter::converter::formatter::{FormatterMode, XCStringFormatter};

fn benchmark_format_single_argument(c: &mut Criterion) {
    let mut formatter = XCStringFormatter::new(FormatterMode::StringUnit);
    let element = AstElement::Argument {
        value: "name1".to_string(),
        span: None,
    };

    c.bench_function("format_single_argument", |b| {
        b.iter(|| {
            let _result = formatter.format(black_box(&element));
        });
    });
}

fn benchmark_format_literal(c: &mut Criterion) {
    let mut formatter = XCStringFormatter::new(FormatterMode::StringUnit);
    let element = AstElement::Literal {
        value: "Hello, World!".to_string(),
        span: None,
    };

    c.bench_function("format_literal", |b| {
        b.iter(|| {
            let _result = formatter.format(black_box(&element));
        });
    });
}

fn benchmark_format_number(c: &mut Criterion) {
    let mut formatter = XCStringFormatter::new(FormatterMode::StringUnit);
    let element = AstElement::Number {
        value: "count".to_string(),
        style: None,
        span: None,
    };

    c.bench_function("format_number", |b| {
        b.iter(|| {
            let _result = formatter.format(black_box(&element));
        });
    });
}

fn benchmark_format_batch_small(c: &mut Criterion) {
    let mut formatter = XCStringFormatter::new(FormatterMode::StringUnit);
    let elements = vec![
        AstElement::Literal {
            value: "Hello, ".to_string(),
            span: None,
        },
        AstElement::Argument {
            value: "name1".to_string(),
            span: None,
        },
        AstElement::Literal {
            value: "!".to_string(),
            span: None,
        },
    ];

    c.bench_function("format_batch_small", |b| {
        b.iter(|| {
            let _result = formatter.format_batch(black_box(&elements));
        });
    });
}

fn benchmark_format_batch_large(c: &mut Criterion) {
    let mut formatter = XCStringFormatter::new(FormatterMode::StringUnit);
    let elements: Vec<AstElement> = (0..100)
        .map(|i| {
            if i % 3 == 0 {
                AstElement::Literal {
                    value: format!("Text {} ", i),
                    span: None,
                }
            } else if i % 3 == 1 {
                AstElement::Argument {
                    value: format!("arg{}", i),
                    span: None,
                }
            } else {
                AstElement::Number {
                    value: format!("num{}", i),
                    style: None,
                    span: None,
                }
            }
        })
        .collect();

    c.bench_function("format_batch_large", |b| {
        b.iter(|| {
            let _result = formatter.format_batch(black_box(&elements));
        });
    });
}

fn benchmark_get_or_insert_position(c: &mut Criterion) {
    let mut formatter = XCStringFormatter::new(FormatterMode::StringUnit);
    let test_values = vec!["arg1", "arg2", "arg3", "arg4", "arg5"];

    c.bench_function("get_or_insert_position", |b| {
        b.iter(|| {
            for value in &test_values {
                let _position = formatter.format(&AstElement::Argument {
                    value: value.to_string(),
                    span: None,
                });
            }
        });
    });
}

fn benchmark_formatter_mode_comparison(c: &mut Criterion) {
    let element = AstElement::Argument {
        value: "name1".to_string(),
        span: None,
    };

    c.bench_function("formatter_string_unit_mode", |b| {
        b.iter(|| {
            let mut formatter = XCStringFormatter::new(FormatterMode::StringUnit);
            let _result = formatter.format(black_box(&element));
        });
    });

    c.bench_function("formatter_plural_mode", |b| {
        b.iter(|| {
            let mut formatter = XCStringFormatter::new(FormatterMode::Plural);
            let _result = formatter.format(black_box(&element));
        });
    });
}

fn benchmark_format_with_capacity(c: &mut Criterion) {
    let mut formatter = XCStringFormatter::new(FormatterMode::StringUnit);
    let element = AstElement::Literal {
        value: "Hello, World! This is a longer string for capacity testing.".to_string(),
        span: None,
    };

    c.bench_function("format_with_capacity_small", |b| {
        b.iter(|| {
            let _result = formatter.format_with_capacity(black_box(&element), 50);
        });
    });

    c.bench_function("format_with_capacity_large", |b| {
        b.iter(|| {
            let _result = formatter.format_with_capacity(black_box(&element), 200);
        });
    });
}

fn benchmark_argument_position_tracking(c: &mut Criterion) {
    c.bench_function("argument_position_tracking_100", |b| {
        b.iter(|| {
            let mut formatter = XCStringFormatter::new(FormatterMode::StringUnit);
            for i in 0..100 {
                let element = AstElement::Argument {
                    value: format!("arg{}", i),
                    span: None,
                };
                let _result = formatter.format(&element);
            }
        });
    });
}

criterion_group!(
    benches,
    benchmark_format_single_argument,
    benchmark_format_literal,
    benchmark_format_number,
    benchmark_format_batch_small,
    benchmark_format_batch_large,
    benchmark_get_or_insert_position,
    benchmark_formatter_mode_comparison,
    benchmark_format_with_capacity,
    benchmark_argument_position_tracking
);
criterion_main!(benches); 