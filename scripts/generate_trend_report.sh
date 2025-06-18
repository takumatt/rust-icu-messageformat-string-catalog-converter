#!/bin/bash

# Benchmark Trend Report Generator
# ベンチマーク結果のトレンド分析とレポート生成

set -e

BENCHMARK_DIR="benchmark_results"
REPORT_DIR="benchmark_reports"
TREND_FILE="$REPORT_DIR/performance_trends.md"

mkdir -p "$REPORT_DIR"

echo "📊 Generating benchmark trend report..."

# ヘッダー作成
cat > "$TREND_FILE" << 'EOF'
# Performance Trend Report

このレポートは、ICU MessageFormat to String Catalog Converterの性能改善の推移を示します。

## 📈 Performance Trends

### Key Metrics Over Time

| Date | Commit | 1000 strings (ms) | 5000 strings (ms) | 10000 strings (ms) | Single Arg (ns) | Literal (ns) |
|------|--------|-------------------|-------------------|-------------------|-----------------|--------------|
EOF

# 結果ファイルを日付順にソートして処理
find "$BENCHMARK_DIR" -name "*.json" -type f | grep -v "latest.json" | grep -v "previous.json" | sort | while read -r file; do
    if [ -f "$file" ]; then
        DATE=$(basename "$file" | cut -d'_' -f1-2 | sed 's/_/ /')
        COMMIT=$(basename "$file" | cut -d'_' -f3)
        
        # JSONから値を抽出
        STRINGS_1000=$(jq -r '.benchmarks.large_file.convert_1000_strings_ms' "$file" 2>/dev/null || echo "N/A")
        STRINGS_5000=$(jq -r '.benchmarks.large_file.convert_5000_strings_ms' "$file" 2>/dev/null || echo "N/A")
        STRINGS_10000=$(jq -r '.benchmarks.large_file.convert_10000_strings_ms' "$file" 2>/dev/null || echo "N/A")
        SINGLE_ARG=$(jq -r '.benchmarks.formatter.format_single_argument_ns' "$file" 2>/dev/null || echo "N/A")
        LITERAL=$(jq -r '.benchmarks.formatter.format_literal_ns' "$file" 2>/dev/null || echo "N/A")
        
        echo "| $DATE | $COMMIT | $STRINGS_1000 | $STRINGS_5000 | $STRINGS_10000 | $SINGLE_ARG | $LITERAL |" >> "$TREND_FILE"
    fi
done

# 統計情報を追加
cat >> "$TREND_FILE" << 'EOF'

## 📊 Statistical Analysis

### Performance Improvements

EOF

# 最新と最古の結果を比較
LATEST_FILE=$(find "$BENCHMARK_DIR" -name "*.json" -type f | grep -v "latest.json" | grep -v "previous.json" | sort | tail -1)
OLDEST_FILE=$(find "$BENCHMARK_DIR" -name "*.json" -type f | grep -v "latest.json" | grep -v "previous.json" | sort | head -1)

if [ -n "$LATEST_FILE" ] && [ -n "$OLDEST_FILE" ] && [ "$LATEST_FILE" != "$OLDEST_FILE" ]; then
    LATEST_1000=$(jq -r '.benchmarks.large_file.convert_1000_strings_ms' "$LATEST_FILE" 2>/dev/null || echo "0")
    OLDEST_1000=$(jq -r '.benchmarks.large_file.convert_1000_strings_ms' "$OLDEST_FILE" 2>/dev/null || echo "0")
    
    if [ "$LATEST_1000" != "0" ] && [ "$OLDEST_1000" != "0" ] && [ "$LATEST_1000" != "null" ] && [ "$OLDEST_1000" != "null" ]; then
        IMPROVEMENT=$(echo "scale=2; (($OLDEST_1000 - $LATEST_1000) / $OLDEST_1000) * 100" | bc -l 2>/dev/null || echo "0")
        echo "- **1000 strings processing**: $OLDEST_1000 ms → $LATEST_1000 ms ($IMPROVEMENT% improvement)" >> "$TREND_FILE"
    fi
fi

# 詳細分析を追加
cat >> "$TREND_FILE" << 'EOF'

### Detailed Metrics

#### Formatter Performance
- **Single Argument**: 引数要素のフォーマット時間
- **Literal**: リテラルテキストのフォーマット時間
- **Batch Processing**: 複数要素の一括処理時間

#### Large File Processing
- **1000 strings**: 小規模ファイルの処理時間
- **5000 strings**: 中規模ファイルの処理時間
- **10000 strings**: 大規模ファイルの処理時間
- **Serialization**: JSONシリアライゼーション時間

## 🎯 Recommendations

### Performance Optimization Opportunities

1. **Formatter Optimization**
   - 引数位置のキャッシュ改善
   - バッチ処理の最適化
   - メモリ割り当ての効率化

2. **Large File Processing**
   - 並列処理の導入
   - ストリーミング処理の実装
   - メモリ使用量の最適化

3. **Monitoring**
   - 継続的なベンチマーク実行
   - 回帰テストの自動化
   - パフォーマンスアラートの設定

## 📋 Usage

### Running Benchmarks
```bash
./scripts/benchmark_tracker.sh
```

### Viewing Trends
```bash
cat benchmark_reports/performance_trends.md
```

### Comparing Results
```bash
diff benchmark_results/previous.json benchmark_results/latest.json
```

---

*Generated on $(date)*
EOF

echo "✅ Trend report generated: $TREND_FILE"

# 簡単な統計サマリーを表示
echo ""
echo "📈 Quick Statistics:"
echo "==================="

if [ -n "$LATEST_FILE" ] && [ -n "$OLDEST_FILE" ] && [ "$LATEST_FILE" != "$OLDEST_FILE" ]; then
    echo "📊 Total benchmark runs: $(find "$BENCHMARK_DIR" -name "*.json" -type f | grep -v "latest.json" | grep -v "previous.json" | wc -l)"
    echo "📅 Date range: $(basename "$OLDEST_FILE" | cut -d'_' -f1-2) to $(basename "$LATEST_FILE" | cut -d'_' -f1-2)"
    
    if [ "$LATEST_1000" != "0" ] && [ "$OLDEST_1000" != "0" ] && [ "$LATEST_1000" != "null" ] && [ "$OLDEST_1000" != "null" ]; then
        echo "🚀 Performance change: $IMPROVEMENT%"
    fi
fi

echo ""
echo "📁 Files created:"
echo "  - Trend report: $TREND_FILE"
echo "  - Raw data: $BENCHMARK_DIR/" 