# rust_file_splicer

指定したディレクトリ以下のファイルを再帰的に探索し、指定した拡張子のファイル内容を一つのテキストファイルに結合するCLIツールです。
ファイルごとに区切り文字を挿入し、どのファイルの内容か分かりやすくします。

**用途:**
* 複数のコードファイルをまとめて解析したいとき
* 対応していない拡張子のファイルをAIに読み込ませたいとき
* ドキュメントを一つにまとめたい場合
* テキストファイルを連結して扱いやすくしたい場合

#### 使用手順
1. コードのビルド
```bash
cargo build --release
```
2. パスを設定する
- Unix系OS (Linux/macOS):
  ```bash
  echo 'export PATH="$PATH:/path/to/target/release"' >> ~/.bashrc  # bashの場合
  echo 'export PATH="$PATH:/path/to/target/release"' >> ~/.zshrc   # zshの場合

**使い方:**
```bash
rust_file_splicer --dir <path> --ext <extension> --output <path>
```

**引数の説明:**
- `--dir`: 探索を開始するディレクトリパス（サブディレクトリも含めて再帰的に探索されます）
- `--ext`: 対象とするファイルの拡張子（例: "rs", "txt"）
- `--output`: 出力先のファイルパス
