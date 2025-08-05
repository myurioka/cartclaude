# タスク完了時のチェックリスト

## コード変更後の必須確認
1. **Rustコードチェック**
   ```bash
   cd src/wasm && cargo check
   ```

2. **WebAssemblyビルド**
   ```bash
   pnpm run build-wasm
   ```

3. **フルビルド確認**
   ```bash
   pnpm run build
   ```

4. **開発サーバーで動作確認**
   ```bash
   pnpm run dev
   # ブラウザで http://localhost:5173 を確認
   ```

## 品質チェック
- Rust警告の解決 (cargo check で確認)
- WebAssemblyコンパイルエラーなし
- ブラウザでゲームが正常動作
- コンソールエラーなし

## コミット前
- 変更内容をテスト
- 不要なファイルが含まれていないか確認
- .gitignore に従った除外確認

## 注意点
- jsディレクトリは現在空なので、wasm-packビルド後に生成される
- ビルドエラーが出る場合は、まずRustコードの型チェックから開始
- Viteビルドエラーは設定ファイルの問題の可能性あり