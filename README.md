# rs_img2pdf

複数の画像ファイルを1つのPDFファイルにまとめるRustコマンドラインツール

## 機能

- 複数の画像ファイル（JPEG、PNG、WebP）を1つのPDFファイルに統合
- ファイル名の数値部分でソート
- WebPファイルは自動的にJPEG（圧縮率80%）に変換
- 各画像のアスペクト比を維持
- 72 DPIでの出力

## サポートしている画像形式

- JPEG (.jpg, .jpeg)
- PNG (.png)
- WebP (.webp)

## インストール

GitHubリポジトリから直接インストール：

```bash
cargo install --git https://github.com/username/rs_img2pdf
```

## 使用方法

### 基本的な使用方法

カレントディレクトリの画像ファイルをPDFに変換：

```bash
rs_img2pdf
```

### オプション指定

```bash
# 入力ディレクトリを指定
rs_img2pdf -i /path/to/images

# 出力ファイル名を指定
rs_img2pdf -o output.pdf

# 入力と出力の両方を指定
rs_img2pdf -i /path/to/images -o /path/to/output.pdf

# 詳細ログを有効化
rs_img2pdf -v

# ログレベルを指定
rs_img2pdf -l debug
```

### コマンドラインオプション

- `-i, --input <DIR>`: 入力ディレクトリ（デフォルト: カレントディレクトリ）
- `-o, --output <FILE>`: 出力PDFファイル（デフォルト: ディレクトリ名.pdf）
- `-l, --log-level <LEVEL>`: ログレベル（trace, debug, info, warn, error）
- `-v, --verbose`: 詳細ログを有効化（debugレベル）
- `-h, --help`: ヘルプを表示
- `-V, --version`: バージョンを表示

## ファイル名のソート

画像ファイルは、ファイル名の数値部分で自動的にソートされます：

- `1.jpg`, `2.jpg`, `10.jpg` → 正しい順序でソート
- `01.jpg`, `02.jpg`, `10.jpg` → 0サプレス対応
- `image1.png`, `image2.png`, `image10.png` → 数値部分でソート

## 開発者向け情報

### ビルド

```bash
git clone https://github.com/username/rs_img2pdf
cd rs_img2pdf
cargo build --release
```

### テスト

```bash
cargo test
```

### 依存関係

- clap: コマンドライン引数解析
- image: 画像処理
- pdf-writer: PDF生成
- log/fern: ログ処理
- walkdir: ディレクトリ検索

## 技術仕様

- **PDF生成**: pdf-writerクレートを使用
- **画像処理**: imageクレートを使用
- **解像度**: 72 DPI固定
- **WebP変換**: JPEG品質80%で変換
- **ページサイズ**: 各画像のアスペクト比に合わせて自動調整

## ライセンス

MIT

## 貢献

プルリクエストやイシューの報告を歓迎します。

## トラブルシューティング

### よくある問題

1. **画像ファイルが見つからない**
   - 対応ファイル形式（jpg, jpeg, png, webp）を確認
   - ファイルの拡張子が正しいか確認

2. **ソート順序が期待と異なる**
   - ファイル名に数値が含まれているか確認
   - 数値以外の文字がある場合はファイル名順でソート

3. **メモリ使用量が大きい**
   - 大きな画像ファイルを処理する際は、システムメモリを確認
