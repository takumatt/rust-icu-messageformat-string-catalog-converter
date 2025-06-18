# ベンチマーク

このディレクトリには、ICU MessageFormat to String Catalog Converterの性能を測定するためのベンチマークが含まれています。

## ベンチマークの種類

### 1. フォーマッターベンチマーク (`formatter_benchmark.rs`)

XCStringFormatterの各メソッドの性能を測定します：

- **format_single_argument**: 単一の引数要素のフォーマット
- **format_literal**: リテラルテキストのフォーマット
- **format_number**: 数値要素のフォーマット
- **format_batch_small**: 小さなバッチ処理（3要素）
- **format_batch_large**: 大きなバッチ処理（100要素）
- **get_or_insert_position**: 引数位置の追跡
- **formatter_mode_comparison**: StringUnitモードとPluralモードの比較
- **format_with_capacity**: 容量指定付きフォーマット
- **argument_position_tracking**: 大量の引数位置追跡

### 2. 大規模ファイルベンチマーク (`large_file_benchmark.rs`)

満単位（1000-10000個）のstringsを処理する際の性能を測定します：

- **convert_1000_strings**: 1000個のstringsの変換
- **convert_5000_strings**: 5000個のstringsの変換
- **convert_10000_strings**: 10000個のstringsの変換
- **convert_with_plurals**: 複数形を含む1000個のstrings
- **convert_with_selects**: select要素を含む1000個のstrings
- **convert_mixed_content**: 混合コンテンツ（通常+複数形+select）
- **memory_usage_analysis**: メモリ使用量の分析
- **serialization_performance**: JSONシリアライゼーション性能

## 実行方法

### 全ベンチマークの実行

```bash
cargo bench
```

### 特定のベンチマークの実行

```bash
# フォーマッターベンチマークのみ
cargo bench --bench formatter_benchmark

# 大規模ファイルベンチマークのみ
cargo bench --bench large_file_benchmark
```

### 特定のベンチマーク関数の実行

```bash
# 1000個のstrings変換のみ
cargo bench --bench large_file_benchmark convert_1000_strings

# 単一引数フォーマットのみ
cargo bench --bench formatter_benchmark format_single_argument
```

## 実際のベンチマーク結果

### フォーマッターベンチマーク結果

```
format_single_argument  time:   [51.161 ns 51.262 ns 51.423 ns]
format_literal          time:   [24.595 ns 24.645 ns 24.703 ns]
format_number           time:   [52.013 ns 52.117 ns 52.237 ns]
format_batch_small      time:   [147.22 ns 148.08 ns 149.22 ns]
format_batch_large      time:   [9.9010 µs 10.016 µs 10.214 µs]
get_or_insert_position  time:   [532.35 ns 534.92 ns 537.96 ns]
formatter_string_unit_mode time: [234.24 ns 235.23 ns 236.31 ns]
formatter_plural_mode   time:   [219.56 ns 220.40 ns 221.36 ns]
format_with_capacity_small time: [148.47 ns 149.15 ns 149.88 ns]
format_with_capacity_large time: [77.541 ns 77.946 ns 78.406 ns]
argument_position_tracking_100 time: [36.817 µs 37.397 µs 38.325 µs]
```

### 大規模ファイルベンチマーク結果

```
convert_1000_strings    time:   [14.724 ms 14.744 ms 14.765 ms]
convert_5000_strings    time:   [76.863 ms 77.129 ms 77.410 ms]
convert_10000_strings   time:   [156.73 ms 158.40 ms 160.43 ms]
convert_1000_strings_with_plurals time: [19.082 ms 19.220 ms 19.445 ms]
convert_1000_strings_with_selects time: [34.323 ms 34.737 ms 35.271 ms]
convert_1000_strings_mixed_content time: [18.630 ms 20.818 ms 23.448 ms]
memory_usage_10000_strings time: [156.76 ms 172.41 ms 193.15 ms]
serialize_5000_strings_to_json time: [2.9719 ms 2.9903 ms 3.0102 ms]
```

## 結果の解釈

### パフォーマンス指標

- **time**: 実行時間（平均、最小、最大）
- **thrpt**: スループット（操作/秒）
- **slope**: 性能の傾向
- **R²**: 統計的信頼性

### 性能分析

#### フォーマッターベンチマーク
- **format_literal**: 最も高速（24.6ns）- 単純な文字列コピー
- **format_single_argument**: 中程度（51.3ns）- 位置計算が必要
- **format_batch_large**: 最も低速（10.0µs）- 大量の要素処理
- **Pluralモード**: StringUnitモードより若干高速（220ns vs 235ns）

#### 大規模ファイルベンチマーク
- **convert_1000_strings**: ~14.7ms（約68,000 strings/秒）
- **convert_5000_strings**: ~77ms（約65,000 strings/秒）
- **convert_10000_strings**: ~158ms（約63,000 strings/秒）
- **複数形処理**: 通常より約30%遅い（19ms vs 15ms）
- **select要素処理**: 通常より約130%遅い（35ms vs 15ms）
- **シリアライゼーション**: 変換時間の約2%（3ms vs 158ms）

### スケーラビリティ

- **線形スケーリング**: 1000→5000→10000で約5倍の時間増加
- **メモリ効率**: 10000個のstringsで約160-200ms
- **シリアライゼーション**: 非常に高速（変換時間の2%未満）

## 最適化のヒント

### フォーマッターの最適化
1. **引数位置のキャッシュ**: 同じ引数を複数回使用する場合
2. **バッチ処理**: 複数要素を一度に処理
3. **容量指定**: 事前に適切な容量を指定

### 大規模ファイル処理の最適化
1. **ストリーミング処理**: メモリ使用量を削減
2. **並列処理**: 複数のstringsを並列に処理
3. **キャッシュ**: パース結果をキャッシュ

## トラブルシューティング

### メモリ不足エラー
```bash
# より少ないサンプル数で実行
cargo bench --bench large_file_benchmark -- --sample-size 10
```

### 長時間の実行
```bash
# タイムアウトを設定
cargo bench --bench large_file_benchmark -- --time-limit 60
```

### 詳細なログ
```bash
# デバッグ情報を表示
RUST_LOG=debug cargo bench --bench large_file_benchmark
```

## 継続的な性能監視

### ベンチマーク結果の保存
```bash
# 結果をJSONファイルに保存
cargo bench --bench large_file_benchmark -- --save-baseline baseline
```

### 結果の比較
```bash
# 前回の結果と比較
cargo bench --bench large_file_benchmark -- --baseline baseline
```

### CI/CDでの実行
```bash
# GitHub Actionsでの実行例
- name: Run benchmarks
  run: cargo bench --bench large_file_benchmark -- --quick
```

## 参考資料

- [Criterion.rs ドキュメント](https://bheisler.github.io/criterion.rs/book/)
- [Rust ベンチマークガイド](https://doc.rust-lang.org/stable/unstable-book/library-features/test.html)
- [ICU MessageFormat 仕様](https://unicode-org.github.io/icu/userguide/format_parse/messages/) 