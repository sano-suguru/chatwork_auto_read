use thiserror::Error;

/// アプリケーション全体で使用されるエラー型を定義します。
///
/// このenumは、アプリケーション内で発生する可能性のある様々な種類のエラーを表現します。
/// `thiserror`クレートを使用して、エラーメッセージの自動生成と他のエラー型からの変換を行います。
#[derive(Error, Debug)]
pub enum Error {
    /// APIリクエスト中に発生したエラーを表します。
    ///
    /// # 引数
    /// * HTTPステータスコード
    /// * エラーメッセージ
    #[error("APIエラー ({0}): {1}")]
    ApiError(reqwest::StatusCode, String),

    /// `reqwest`ライブラリのエラーをラップします。
    #[error("Reqwestエラー: {0}")]
    ReqwestError(#[from] reqwest::Error),

    /// 環境変数の取得中に発生したエラーを表します。
    #[error("環境変数エラー: {0}")]
    EnvVarError(#[from] std::env::VarError),

    /// JSON解析中に発生したエラーを表します。
    #[error("Serde JSONエラー: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    /// 設定ファイルの読み込み中に発生したエラーを表します。
    #[error("設定エラー: {0}")]
    ConfigError(#[from] config::ConfigError),

    /// リトライ回数が最大値に達したことを表します。
    #[error("最大リトライ回数を超過しました")]
    MaxRetriesExceeded,

    /// 入出力操作中に発生したエラーを表します。
    #[error("I/Oエラー: {0}")]
    IoError(#[from] std::io::Error),

    /// その他の予期しないエラーを表します。
    ///
    /// `anyhow::Error`を使用して、様々な種類のエラーを捕捉します。
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
