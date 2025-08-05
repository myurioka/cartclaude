# コードベース構造

## ディレクトリ構成
```
cartclaude/
├── src/wasm/           # Rustゲームエンジン
│   ├── src/
│   │   ├── lib.rs      # メインエントリーポイント
│   │   ├── engine.rs   # ゲームエンジン (GameLoop, Renderer等)
│   │   ├── game.rs     # ゲームロジック (GameStage, ステートマシン)
│   │   ├── browser.rs  # ブラウザAPI ラッパー
│   │   ├── sound.rs    # 音声処理
│   │   └── game/       # ゲーム要素
│   │       ├── cart.rs     # カート
│   │       ├── wall.rs     # 壁
│   │       ├── music.rs    # 音楽
│   │       └── ornament.rs # 装飾
│   ├── Cargo.toml      # Rust依存関係
│   └── pkg/            # wasm-packビルド出力
├── js/                 # JavaScript (ビルド後)
├── static/             # 静的ファイル (CSS, フォント)
├── index.html          # HTMLエントリーポイント
└── package.json        # Node.js依存関係
```

## アーキテクチャ
- **ステートマシンパターン**: ゲーム状態管理
- **ECS風設計**: エンティティとコンポーネントの分離
- **非同期処理**: futures を使用したイベントループ