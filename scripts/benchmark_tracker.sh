#!/bin/bash

# Benchmark Tracker Script
# 継続的なベンチマーク結果の追跡と保存

set -e

# 設定
BENCHMARK_DIR="benchmark_results"
CURRENT_COMMIT=$(git rev-parse HEAD)
CURRENT_DATE=$(date +"%Y-%m-%d_%H-%M-%S")
MACHINE_INFO=$(uname -s)_$(uname -m)
RESULT_FILE="${BENCHMARK_DIR}/${CURRENT_DATE}_${CURRENT_COMMIT:0:8}_${MACHINE_INFO}.json"

# ディレクトリ作成
mkdir -p "$BENCHMARK_DIR"

echo "🚀 Starting benchmark tracking..."
echo "📅 Date: $CURRENT_DATE"
echo "🔗 Commit: $CURRENT_COMMIT"
echo "💻 Machine: $MACHINE_INFO"
echo "📁 Results: $RESULT_FILE"

# ベンチマーク結果をJSON形式で保存
cat > "$RESULT_FILE" << EOF
{
  "metadata": {
    "date": "$CURRENT_DATE",
    "commit_hash": "$CURRENT_COMMIT",
    "commit_short": "${CURRENT_COMMIT:0:8}",
    "machine": "$MACHINE_INFO",
    "rust_version": "$(rustc --version | cut -d' ' -f2)",
    "cargo_version": "$(cargo --version | cut -d' ' -f2)"
  },
  "benchmarks": {
EOF

# フォーマッターベンチマーク実行
echo "📊 Running formatter benchmarks..."
FORMATTER_OUTPUT=$(cargo bench --bench formatter_benchmark 2>&1)

# 大規模ファイルベンチマーク実行
echo "📊 Running large file benchmarks..."
LARGE_FILE_OUTPUT=$(cargo bench --bench large_file_benchmark 2>&1)

# 結果を手動で抽出（Criterionの出力から）
FORMATTER_SINGLE_ARG=$(echo "$FORMATTER_OUTPUT" | grep "format_single_argument" | grep -oE "time: \[[0-9.]+ [a-z]+ [0-9.]+ [a-z]+\]" | head -1 | grep -oE "[0-9.]+" | head -1)
FORMATTER_LITERAL=$(echo "$FORMATTER_OUTPUT" | grep "format_literal" | grep -oE "time: \[[0-9.]+ [a-z]+ [0-9.]+ [a-z]+\]" | head -1 | grep -oE "[0-9.]+" | head -1)
FORMATTER_BATCH_LARGE=$(echo "$FORMATTER_OUTPUT" | grep "format_batch_large" | grep -oE "time: \[[0-9.]+ [a-z]+ [0-9.]+ [a-z]+\]" | head -1 | grep -oE "[0-9.]+" | head -1)

LARGE_1000=$(echo "$LARGE_FILE_OUTPUT" | grep "convert_1000_strings" | grep -oE "time: \[[0-9.]+ [a-z]+ [0-9.]+ [a-z]+\]" | head -1 | grep -oE "[0-9.]+" | head -1)
LARGE_5000=$(echo "$LARGE_FILE_OUTPUT" | grep "convert_5000_strings" | grep -oE "time: \[[0-9.]+ [a-z]+ [0-9.]+ [a-z]+\]" | head -1 | grep -oE "[0-9.]+" | head -1)
LARGE_10000=$(echo "$LARGE_FILE_OUTPUT" | grep "convert_10000_strings" | grep -oE "time: \[[0-9.]+ [a-z]+ [0-9.]+ [a-z]+\]" | head -1 | grep -oE "[0-9.]+" | head -1)
LARGE_SERIALIZE=$(echo "$LARGE_FILE_OUTPUT" | grep "serialize_5000_strings_to_json" | grep -oE "time: \[[0-9.]+ [a-z]+ [0-9.]+ [a-z]+\]" | head -1 | grep -oE "[0-9.]+" | head -1)

# デフォルト値を設定
FORMATTER_SINGLE_ARG=${FORMATTER_SINGLE_ARG:-0}
FORMATTER_LITERAL=${FORMATTER_LITERAL:-0}
FORMATTER_BATCH_LARGE=${FORMATTER_BATCH_LARGE:-0}
LARGE_1000=${LARGE_1000:-0}
LARGE_5000=${LARGE_5000:-0}
LARGE_10000=${LARGE_10000:-0}
LARGE_SERIALIZE=${LARGE_SERIALIZE:-0}

# 結果をJSONに追加
cat >> "$RESULT_FILE" << EOF
    "formatter": {
      "format_single_argument_ns": $FORMATTER_SINGLE_ARG,
      "format_literal_ns": $FORMATTER_LITERAL,
      "format_number_ns": 0,
      "format_batch_small_ns": 0,
      "format_batch_large_us": $FORMATTER_BATCH_LARGE,
      "get_or_insert_position_ns": 0,
      "formatter_string_unit_mode_ns": 0,
      "formatter_plural_mode_ns": 0,
      "format_with_capacity_small_ns": 0,
      "format_with_capacity_large_ns": 0,
      "argument_position_tracking_100_us": 0
    },
    "large_file": {
      "convert_1000_strings_ms": $LARGE_1000,
      "convert_5000_strings_ms": $LARGE_5000,
      "convert_10000_strings_ms": $LARGE_10000,
      "convert_1000_strings_with_plurals_ms": 0,
      "convert_1000_strings_with_selects_ms": 0,
      "convert_1000_strings_mixed_content_ms": 0,
      "memory_usage_10000_strings_ms": 0,
      "serialize_5000_strings_to_json_ms": $LARGE_SERIALIZE
    }
  }
}
EOF

echo "✅ Benchmark results saved to $RESULT_FILE"

# 最新の結果をシンボリックリンクで参照
ln -sf "$RESULT_FILE" "$BENCHMARK_DIR/latest.json"

# 結果の要約を表示
echo ""
echo "📈 Benchmark Summary:"
echo "===================="
echo "Formatter Benchmarks:"
echo "  - Single argument: ${FORMATTER_SINGLE_ARG} ns"
echo "  - Literal: ${FORMATTER_LITERAL} ns"
echo "  - Batch large: ${FORMATTER_BATCH_LARGE} μs"
echo ""
echo "Large File Benchmarks:"
echo "  - 1000 strings: ${LARGE_1000} ms"
echo "  - 5000 strings: ${LARGE_5000} ms"
echo "  - 10000 strings: ${LARGE_10000} ms"
echo "  - Serialization: ${LARGE_SERIALIZE} ms"

# 前回の結果と比較（存在する場合）
if [ -f "$BENCHMARK_DIR/latest.json" ] && [ -f "$BENCHMARK_DIR/previous.json" ]; then
    echo ""
    echo "📊 Performance Comparison:"
    echo "========================="
    
    # 主要な指標を比較
    PREVIOUS_1000=$(jq -r '.benchmarks.large_file.convert_1000_strings_ms' "$BENCHMARK_DIR/previous.json" 2>/dev/null || echo "0")
    
    if [ "$PREVIOUS_1000" != "0" ] && [ "$PREVIOUS_1000" != "null" ] && [ "$LARGE_1000" != "0" ]; then
        IMPROVEMENT=$(echo "scale=2; (($PREVIOUS_1000 - $LARGE_1000) / $PREVIOUS_1000) * 100" | bc -l 2>/dev/null || echo "0")
        echo "  1000 strings: $PREVIOUS_1000 ms → $LARGE_1000 ms ($IMPROVEMENT% change)"
    fi
fi

# 現在の結果を前回の結果として保存
cp "$RESULT_FILE" "$BENCHMARK_DIR/previous.json"

echo ""
echo "🎯 Next steps:"
echo "  - View detailed results: cat $RESULT_FILE"
echo "  - Compare with previous: diff $BENCHMARK_DIR/previous.json $RESULT_FILE"
echo "  - Generate trend report: ./scripts/generate_trend_report.sh" 