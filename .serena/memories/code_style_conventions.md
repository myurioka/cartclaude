# コードスタイルと規約

## Rust規約
- **Edition**: 2024
- **命名規則**: snake_case (変数、関数), PascalCase (型、構造体)
- **モジュール**: 機能別に分割 (engine, game, browser, sound)
- **定数**: UPPER_SNAKE_CASE (例: CANVAS_WIDTH, FONT_COLOR)

## プロジェクト固有パターン
- **ステートマシン**: GameStage + State<T> パターン使用
- **非同期処理**: futures + wasm-bindgen-futures
- **エラーハンドリング**: anyhow::Result<T> + console_error_panic_hook
- **Web API**: web-sys を通じたDOM操作

## ファイル構成規約
- `mod.rs` ではなく、ファイル名でモジュール定義
- game/ サブディレクトリでゲーム要素を分離
- 各モジュールは単一責任を持つ

## WebAssembly特有
- `#[wasm_bindgen]` 属性でJavaScript公開
- `JsValue` を返り値として使用
- `console_error_panic_hook::set_once()` でデバッグ支援

## 現在の警告
- `associated function 'new' is never used` in engine.rs:41
  - 未使用のコードは適宜削除またはテストで使用