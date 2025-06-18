# ベンチマーク追跡システム

このディレクトリには、ICU MessageFormat to String Catalog Converterの性能を継続的に追跡するためのスクリプトが含まれています。

## 📁 スクリプト一覧

### 1. `benchmark_tracker.sh`
メインのベンチマーク実行スクリプトです。

**機能:**
- フォーマッターベンチマークと大規模ファイルベンチマークを実行
- 結果をJSON形式で保存（コミットハッシュ、日時、マシン情報付き）
- 前回の結果との比較
- 結果の要約表示

**使用方法:**
```bash
./scripts/benchmark_tracker.sh
```

**出力:**
- `benchmark_results/YYYY-MM-DD_HH-MM-SS_COMMIT_MACHINE.json`
- `benchmark_results/latest.json` (シンボリックリンク)
- `benchmark_results/previous.json` (前回の結果)

### 2. `generate_trend_report.sh`
ベンチマーク結果のトレンド分析レポートを生成します。

**機能:**
- 全ベンチマーク結果の時系列分析
- 性能改善の統計情報
- マークダウン形式のレポート生成

**使用方法:**
```bash
./scripts/generate_trend_report.sh
```

**出力:**
- `benchmark_reports/performance_trends.md`

### 3. `ci_benchmark.sh`
CI/CD環境用の簡易ベンチマークスクリプトです。

**機能:**
- 軽量なベンチマーク実行
- 基本的なパフォーマンスチェック
- CI環境での使用に最適化

**使用方法:**
```bash
./scripts/ci_benchmark.sh
```

## 🚀 クイックスタート

### 初回実行
```bash
# スクリプトに実行権限を付与
chmod +x scripts/*.sh

# ベンチマーク実行
./scripts/benchmark_tracker.sh

# トレンドレポート生成
./scripts/generate_trend_report.sh
```

### 継続的な使用
```bash
# 開発後にベンチマーク実行
git commit -m "feat: some improvement"
./scripts/benchmark_tracker.sh

# 結果を確認
cat benchmark_results/latest.json

# 前回との比較
diff benchmark_results/previous.json benchmark_results/latest.json

# トレンドレポート更新
./scripts/generate_trend_report.sh
```

## 📊 結果の解釈

### ファイル構造
```
benchmark_results/
├── 2024-01-15_10-30-45_a1b2c3d4_Darwin_arm64.json  # 個別の結果
├── latest.json                                       # 最新結果へのリンク
└── previous.json                                     # 前回の結果

benchmark_reports/
└── performance_trends.md                             # トレンドレポート
```

### JSON結果の構造
```json
{
  "metadata": {
    "date": "2024-01-15_10-30-45",
    "commit_hash": "a1b2c3d4e5f6...",
    "commit_short": "a1b2c3d4",
    "machine": "Darwin_arm64",
    "rust_version": "1.87.0",
    "cargo_version": "1.87.0"
  },
  "benchmarks": {
    "formatter": {
      "format_single_argument_ns": 51.2,
      "format_literal_ns": 24.6,
      // ... その他のフォーマッター指標
    },
    "large_file": {
      "convert_1000_strings_ms": 14.7,
      "convert_5000_strings_ms": 77.1,
      // ... その他の大規模ファイル指標
    }
  }
}
```

## 📈 性能追跡のベストプラクティス

### 1. 定期的な実行
```bash
# 開発セッションの開始時
./scripts/benchmark_tracker.sh

# 重要な変更後
git commit -m "perf: optimize formatter"
./scripts/benchmark_tracker.sh

# 週次レビュー
./scripts/generate_trend_report.sh
```

### 2. 結果の分析
```bash
# 最新の結果を確認
jq '.benchmarks.large_file.convert_1000_strings_ms' benchmark_results/latest.json

# 性能改善を確認
jq '.benchmarks.large_file.convert_1000_strings_ms' benchmark_results/previous.json
jq '.benchmarks.large_file.convert_1000_strings_ms' benchmark_results/latest.json

# 全履歴を確認
ls -la benchmark_results/*.json | grep -v latest | grep -v previous
```

### 3. アラート設定
```bash
# 性能劣化の検出
CURRENT=$(jq -r '.benchmarks.large_file.convert_1000_strings_ms' benchmark_results/latest.json)
PREVIOUS=$(jq -r '.benchmarks.large_file.convert_1000_strings_ms' benchmark_results/previous.json)
THRESHOLD=1.2  # 20%の劣化を許容

if (( $(echo "$CURRENT > $PREVIOUS * $THRESHOLD" | bc -l) )); then
    echo "⚠️  Performance regression detected!"
    echo "Previous: ${PREVIOUS}ms, Current: ${CURRENT}ms"
fi
```

## 🔧 カスタマイズ

### ベンチマーク設定の変更
`Cargo.toml`の`[[bench]]`セクションでベンチマークの設定を調整できます。

### 結果保存のカスタマイズ
`benchmark_tracker.sh`の`BENCHMARK_DIR`変数を変更して保存場所を変更できます。

### 比較ロジックの調整
`generate_trend_report.sh`で比較ロジックやレポート形式をカスタマイズできます。

## 🐛 トラブルシューティング

### 依存関係の問題
```bash
# jqのインストール（macOS）
brew install jq

# bcのインストール（macOS）
brew install bc

# Ubuntu/Debian
sudo apt-get install jq bc
```

### 権限の問題
```bash
chmod +x scripts/*.sh
```

### メモリ不足
```bash
# より少ないサンプル数で実行
cargo bench --bench large_file_benchmark -- --sample-size 10
```

## 📋 参考資料

- [Criterion.rs ドキュメント](https://bheisler.github.io/criterion.rs/book/)
- [jq マニュアル](https://stedolan.github.io/jq/manual/)
- [ベンチマーク結果の解釈](benches/README.md) 