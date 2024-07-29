# 🤖 Chatwork Auto Read

![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)

## 📖 概要

Chatwork Auto Read は、Chatwork のメッセージを自動的に既読化するアプリケーションです。

## 🌟 主な機能

- **自動既読化**: 新着メッセージを自動で既読にします
- **スキップ機能**: 特定の条件でメッセージ処理をスキップできます

## 🚀 使用方法

### 💾 プログラムのダウンロードと使い方

1. プログラムのダウンロード:

   - [ダウンロードページ](https://github.com/sano-suguru/chatwork_auto_read/releases)にアクセスします。
   - 最新版の下にある「Assets」という見出しを探します。
   - お使いのパソコンに合ったファイルをダウンロードします：
     - Windows パソコンの方: `chatwork_auto_read-Windows.exe`
     - Mac の方: `chatwork_auto_read-macOS`
     - Linux の方: `chatwork_auto_read-Linux`

2. ダウンロードしたファイルの保存:

   - パソコンの分かりやすい場所（例：デスクトップ）に新しいフォルダを作り、その中にダウンロードしたファイルを移動させます。
   - このフォルダを「Chatwork 自動既読」などと名付けると良いかと思います。

3. Chatwork API トークンの取得:

   - Chatwork API トークンが必要です。取得方法は以下の手順で確認できます：
     - [Chatwork 公式ヘルプページ「API トークンを発行する」](https://help.chatwork.com/hc/ja/articles/115000172402-APIトークンを発行する) にアクセスします。
     - このページの手順に従って、自分の Chatwork API トークンを取得してください。
   - 取得した API トークンは、次の手順で使用しますので、メモしておいてください。

4. 設定ファイルの作成:

   - ダウンロードしたプログラムと同じフォルダに、`config`というフォルダを作ります。
   - `config`フォルダの中に`default.toml`というファイルを作ります。
   - `default.toml`ファイルを開き、以下のような内容を入力します：

     ```toml
     [chatwork]
     api_token = "ここにあなたのAPIトークンを入れてください"
     exclude_account_ids = ["あなたのアカウントID", "その他既読にしたくないメンションのアカウントID1",, "その他既読にしたくないメンションのアカウントID2"]
     exclude_room_ids = [スキップしたいルームID1, スキップしたいルームID2]
     ```

   設定例：

   ```toml
   [chatwork]
   api_token = "abcdef1234567890ghijklmnopqrstuvwxyz"
   exclude_account_ids = ["123456","7891011"]
   exclude_room_ids = [11111, 22222]
   ```

   この例では：

   - API トークンは `"abcdef1234567890ghijklmnopqrstuvwxyz"` です（これは架空のものです。実際のトークンに置き換えてください）。
   - 自分のアカウント ID（123456）が含まれるメッセージは自動既読にされません。
   - ルーム ID 11111 と 22222 のメッセージは自動既読にされません。

   注意：

   - アカウント ID はかぎかっこ（`[]`）の中にダブルクォーテーション（`"`）で囲んで入力します。
   - ルーム ID はかぎかっこ（`[]`）の中に直接数字を入力します。
   - 複数のアカウント ID やルーム ID を設定する場合は、カンマ（`,`）で区切ります。

   アカウント ID の確認方法：

   - [Chatwork 公式ヘルプページ「アカウント ID を確認する」](https://help.chatwork.com/hc/ja/articles/360000142962-アカウントIDを確認する)の手順に従って確認できます。
     簡単な手順：
     1. Chatwork のウェブサイトにログインします。
     2. 画面右上のアイコンをクリックし、「アカウント設定」を選択します。
     3. 「プロフィール」タブを開くと、「アカウント ID」が表示されています。

   ルーム ID の確認方法：

   - [Chatwork 公式ヘルプページ「ルーム ID を確認する」](https://help.chatwork.com/hc/ja/articles/360000142942-ルームIDを確認する)の手順に従って確認できます。
     簡単な手順：
     1. Chatwork のウェブサイトでルームを開きます。
     2. ブラウザのアドレスバーに表示される URL の末尾の数字がルーム ID です。
        例：`https://www.chatwork.com/#!rid00000` の `00000` 部分がルーム ID です。

5. プログラムの実行:

   Windows の方:

   - 「Chatwork 自動既読」フォルダ内の `chatwork_auto_read-Windows.exe` をダブルクリックします。
   - 「Windows によって PC が保護されました」という警告が出た場合は、「詳細情報」をクリックし、「実行」を選んでください。

   Mac の方:

   - 「ターミナル」アプリを開きます（アプリケーション → ユーティリティ → ターミナル）。
   - 以下のコマンドを順番に入力します（各行を入力後、Enter キーを押します）：
     ```
     cd デスクトップ/Chatwork自動既読
     chmod +x ./chatwork_auto_read-macOS
     ./chatwork_auto_read-macOS
     ```
   - 「開発元を確認できないため開けません」という警告が出た場合は、「システム環境設定」→「セキュリティとプライバシー」から、「このまま開く」を選択してください。

   Linux の方:

   - ターミナルを開きます。
   - 以下のコマンドを順番に入力します（各行を入力後、Enter キーを押します）：
     ```
     cd ~/デスクトップ/Chatwork自動既読
     chmod +x ./chatwork_auto_read-Linux
     ./chatwork_auto_read-Linux
     ```

注意: プログラムを初めて実行する際、お使いのパソコンのセキュリティ設定により警告が表示される場合があります。これは正常な動作です。上記の手順に従って許可を与えてください。

---

## 👨‍💻 開発者向け情報

### 📚 API ドキュメント

詳細な API ドキュメントは[こちら](https://sano-suguru.github.io/chatwork_auto_read/doc/chatwork_auto_read/index.html)でご覧いただけます。

### 🛠️ 技術スタック

- **言語**: Rust 1.70+
- **非同期処理**: tokio
- **HTTP 通信**: reqwest
- **シリアライゼーション**: serde
- **設定管理**: config
- **ログ管理**: log, env_logger
- **エラー処理**: anyhow, thiserror
- **非同期トレイト**: async-trait
- **テスト**: mockall

### 🔧 技術的特徴

- **効率的な非同期処理**: tokio を使用した高性能な非同期処理
- **堅牢なエラーハンドリング**: anyhow と thiserror を用いた包括的なエラー管理
- **レート制限対応**: API レート制限に対する自動再試行機能
- **テスト駆動開発**: mockall を使用した単体テストの実装

### 🔨 セットアップ

1. リポジトリのクローン:

   ```sh
   git clone https://github.com/sano-suguru/chatwork_auto_read.git
   cd chatwork_auto_read
   ```

2. 依存関係のインストール:
   ```sh
   cargo build
   ```

### ⚙️ 設定

1. `config/default.toml` ファイルを作成:

   ```toml
   [chatwork]
   api_token = "YOUR_CHATWORK_API_TOKEN"
   exclude_account_ids = ["TARGET_ID_1", "TARGET_ID_2"]
   exclude_room_ids = [ROOM_ID_1, ROOM_ID_2]
   ```

2. 必要に応じて環境別の設定ファイル（例：`config/production.toml`）を作成

### 🏃‍♂️ 実行

基本的な実行:

```sh
./chatwork_auto_read
```

環境別の設定を使用:

```sh
RUN_MODE=production ./chatwork_auto_read
```

ログレベルの調整:

```sh
RUST_LOG=debug ./chatwork_auto_read
```

### 🧪 テスト

単体テストの実行:

```sh
cargo test
```

### 📁 プロジェクト構造

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

### 💡 開発のポイント

1. **モジュラー設計**: 機能ごとに分離されたモジュール構造
2. **非同期プログラミング**: `async/await`を活用した効率的な非同期処理
3. **エラーハンドリング**: カスタムエラー型と`Result`を用いた包括的なエラー管理
4. **設定の柔軟性**: 環境変数と設定ファイルを組み合わせた適応性の高い設定システム
5. **API レート制限対策**: 指数バックオフアルゴリズムによる再試行メカニズム
6. **ログ管理**: 詳細なログ記録による運用性の向上
7. **テスト駆動開発**: モックを使用した包括的な単体テスト
8. **スキップロジック**: 特定の条件に基づいてメッセージ処理をスキップする機能

### 📈 今後の展望

- CI/CD パイプラインの構築
- パフォーマンスの最適化
- より詳細なドキュメンテーション
- コードカバレッジの向上

### 📄 ライセンス

このプロジェクトは [MIT ライセンス](https://opensource.org/license/mit) の下で公開されています。

---

- 👨‍💻 開発者: [@sano-suguru](https://github.com/sano-suguru)
