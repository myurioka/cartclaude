# 開発ガイドライン

## 設計パターン
- **ステートマシンパターン**: ゲーム状態 (Ready, Playing, GameOver, GameClear) の管理
- **From トレイト**: 状態遷移の実装
- **非同期プログラミング**: futures + spawn_local
- **モジュラー設計**: 機能別モジュール分割

## パフォーマンス考慮
- WebAssembly最適化: wasm-opt = false (デバッグ用)
- Canvas描画: requestAnimationFrame を使用
- 音声: AudioContext + AudioBuffer

## ブラウザ互換性
- web-sys を通じたWeb API使用
- Canvas対応必須
- AudioContext 対応推奨

## デバッグ戦略
- `console_error_panic_hook` でRustパニックをJSコンソールに表示
- `browser::log()` マクロでログ出力
- ブラウザ開発者ツールでデバッグ

## セキュリティ
- WebAssemblyサンドボックス内で実行
- DOM操作は制限されたAPIのみ
- 外部リソース読み込みは慎重に

## 追加機能開発時の注意
- 新しいWeb API使用時はCargo.tomlのweb-sys featuresに追加
- ゲーム状態追加時はステートマシンパターンに従う
- 音声ファイル追加時は適切な形式 (WebAudio対応) を使用