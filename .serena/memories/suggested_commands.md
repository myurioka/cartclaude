# 推奨コマンド

## 開発コマンド
- `pnpm run dev` - 開発サーバー起動
- `pnpm run build` - プロダクションビルド
- `pnpm run build-wasm` - WebAssemblyのみビルド
- `pnpm run preview` - プロダクションプレビュー

## Rustコマンド (src/wasm/ ディレクトリ内)
- `cargo check` - Rustコードの型チェック
- `cargo build` - Rustコードのビルド
- `cargo test` - テスト実行
- `wasm-pack build --target web` - WebAssemblyビルド

## 便利なシステムコマンド
- `ls` - ファイル一覧
- `find . -name "*.rs"` - Rustファイル検索
- `grep -r "pattern" src/` - コード検索
- `git status` - Git状態確認
- `git add .` - 変更をステージング
- `git commit -m "message"` - コミット

## ビルド確認
開発時は以下の順序で確認：
1. `cd src/wasm && cargo check` - Rust型チェック
2. `pnpm run build-wasm` - WebAssemblyビルド
3. `pnpm run dev` - 開発サーバーで動作確認