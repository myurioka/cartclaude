# 技術スタック

## フロントエンド
- **HTML5**: Canvas要素を使用したゲーム描画
- **CSS**: スタイリング (static/main.css)
- **JavaScript**: WebAssemblyとの接続 (js/main.js)

## バックエンド/ゲームエンジン
- **Rust**: メインのゲームロジック
- **WebAssembly (WASM)**: ブラウザでのRust実行
- **wasm-bindgen**: RustとJavaScript間のバインディング
- **web-sys**: Web API へのアクセス

## ビルドツール
- **Vite**: 開発サーバーとビルドツール
- **wasm-pack**: WebAssemblyパッケージビルダー
- **pnpm**: パッケージマネージャー

## 主要依存関係
- console_error_panic_hook: エラーハンドリング
- rand: 乱数生成
- futures: 非同期処理
- js-sys, web-sys: Web API バインディング
- serde: シリアライゼーション