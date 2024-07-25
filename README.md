# Chatwork Auto Read

![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)

## 📖 概要

Chatwork Auto Read は、Chatwork API を活用して特定のメッセージを自動的に既読化する Rust アプリケーションです。非同期処理、エラーハンドリング、設定管理など、Rust の先進的な開発手法を実践しています。

### 🌟 主な特徴

- **自動既読化**: 指定された条件に基づき、メッセージを自動で既読にします
- **柔軟な設定**: 環境別の設定ファイルと環境変数によるカスタマイズが可能
- **効率的な非同期処理**: tokio を使用した高性能な非同期処理
- **堅牢なエラーハンドリング**: anyhow と thiserror を用いた包括的なエラー管理
- **レート制限対応**: API レート制限に対する自動再試行機能
- **テスト駆動開発**: mockall を使用した単体テストの実装
- **スキップ機能**: 特定の条件（メンションがある場合や指定されたルーム ID）に基づいてメッセージ処理をスキップ

## 🛠️ 技術スタック

- **言語**: Rust 1.70+
- **非同期処理**: tokio
- **HTTP 通信**: reqwest
- **シリアライゼーション**: serde
- **設定管理**: config
- **ログ管理**: log, env_logger
- **エラー処理**: anyhow, thiserror
- **非同期トレイト**: async-trait
- **テスト**: mockall

## 🚀 セットアップと使用方法

### 前提条件

- Rust 1.70 以上
- Cargo (Rust のパッケージマネージャ)

### インストール

1. リポジトリのクローン:

   ```sh
   git clone https://github.com/sano-suguru/chatwork_auto_read.git
   cd chatwork_auto_read
   ```

2. 依存関係のインストール:
   ```sh
   cargo build
   ```

### 設定

1. `config/default.toml` ファイルを作成:

   ```toml
   [chatwork]
   api_token = "YOUR_CHATWORK_API_TOKEN"
   skip_account_ids = ["TARGET_ID_1", "TARGET_ID_2"]
   skip_room_ids = [ROOM_ID_1, ROOM_ID_2]
   ```

2. 必要に応じて環境別の設定ファイル（例：`config/production.toml`）を作成

### 実行

基本的な実行:

```sh
cargo run
```

環境別の設定を使用:

```sh
RUN_MODE=production cargo run
```

ログレベルの調整:

```sh
RUST_LOG=debug cargo run
```

### テスト

単体テストの実行:

```sh
cargo test
```

## 📁 プロジェクト構造

```
src/
├── main.rs          # アプリケーションのエントリーポイント
├── lib.rs           # ライブラリのエントリーポイント
├── client/
│   └── chatwork.rs  # Chatwork API クライアント
├── models/
│   ├── message.rs   # メッセージモデル
│   └── room.rs      # ルームモデル
├── error.rs         # エラー定義
├── settings.rs      # 設定管理
├── processor.rs     # メッセージ処理ロジック
└── utils.rs         # ユーティリティ関数（ログ設定など）
```

## 💡 開発のポイント

1. **モジュラー設計**: 機能ごとに分離されたモジュール構造
2. **非同期プログラミング**: `async/await`を活用した効率的な非同期処理
3. **エラーハンドリング**: カスタムエラー型と`Result`を用いた包括的なエラー管理
4. **設定の柔軟性**: 環境変数と設定ファイルを組み合わせた適応性の高い設定システム
5. **API レート制限対策**: 指数バックオフアルゴリズムによる再試行メカニズム
6. **ログ管理**: 詳細なログ記録による運用性の向上
7. **テスト駆動開発**: モックを使用した包括的な単体テスト
8. **スキップロジック**: 特定の条件に基づいてメッセージ処理をスキップする機能

## 📈 今後の展望

- CI/CD パイプラインの構築
- パフォーマンスの最適化
- より詳細なドキュメンテーション
- コードカバレッジの向上

## 📄 ライセンス

このプロジェクトは [MIT ライセンス](https://opensource.org/license/mit) の下で公開されています。

---

- 👨‍💻 開発者: @sano-suguru
- 🌐 ポートフォリオ: [準備中]
