//! Chatworkメッセージ処理システムのルートモジュールです。
//!
//! このモジュールは、アプリケーションの主要なコンポーネントをまとめ、
//! 実行のエントリーポイントとなる`run`関数を提供します。

/// Chatwork APIクライアントの実装を含むモジュールです。
pub mod client;
/// エラー型の定義を含むモジュールです。
pub mod error;
/// データモデルの定義を含むモジュールです。
pub mod models;
/// メッセージ処理ロジックを含むモジュールです。
pub mod processor;
/// アプリケーション設定の管理を行うモジュールです。
pub mod settings;
/// ユーティリティ関数を含むモジュールです。
pub mod utils;

pub use client::ChatworkClient;
pub use error::Error;
pub use processor::MessageProcessor;
pub use settings::Settings;

use anyhow::Result;

/// アプリケーションのメイン実行関数です。
///
/// この関数は以下の手順を実行します：
/// 1. ロギングシステムのセットアップ
/// 2. アプリケーション設定の読み込み
/// 3. Chatworkクライアントの初期化
/// 4. メッセージプロセッサの作成
/// 5. 全ルームのメッセージ処理の実行
///
/// # エラー
///
/// 設定の読み込みやメッセージ処理中にエラーが発生した場合、
/// `anyhow::Error`でラップされたエラーを返します。
///
/// # 例
///
/// ```
/// use your_crate_name::run;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     run().await
/// }
/// ```
pub async fn run() -> Result<()> {
    utils::setup_logging();

    let settings = Settings::new()?;
    let client = ChatworkClient::new(&settings.chatwork.api_token);
    let processor = MessageProcessor::new(client, settings);

    processor.process_all_rooms().await?;

    Ok(())
}
