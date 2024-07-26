use crate::error::Error;
use crate::models::{Message, ReadStatus, Room};
use anyhow::Context;
use async_trait::async_trait;
use log::{error, info, warn};
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;
use tokio::time::sleep;

use mockall::automock;

/// APIリクエストの最大リトライ回数。
const MAX_RETRY_ATTEMPTS: usize = 5;
/// リトライ間の初期遅延時間。
const INITIAL_RETRY_DELAY: Duration = Duration::from_secs(10);

/// Chatwork APIとの対話のためのインターフェースを定義します。
///
/// このトレイトはルーム、メッセージの取得、およびメッセージを既読としてマークするためのメソッドを提供します。
/// 具体的なクライアントタイプによって実装されるように設計されており、テストのために簡単にモック化できます。
#[automock]
#[async_trait]
pub trait ChatworkClientTrait {
    /// 認証されたユーザーがアクセス可能な全てのルームを取得します。
    ///
    /// # 戻り値
    ///
    /// 成功した場合は`Room`オブジェクトのベクターを含む`Result`、操作が失敗した場合は`Error`を返します。
    async fn fetch_rooms(&self) -> Result<Vec<Room>, Error>;

    /// 特定のルームからメッセージを取得します。
    ///
    /// # 引数
    ///
    /// * `room_id` - メッセージを取得するルームのID。
    ///
    /// # 戻り値
    ///
    /// 成功した場合は`Message`オブジェクトのベクターを含む`Result`、操作が失敗した場合は`Error`を返します。
    async fn fetch_messages(&self, room_id: i32) -> Result<Vec<Message>, Error>;

    /// 指定されたルーム内の特定のメッセージを既読としてマークします。
    ///
    /// このメソッドは、Chatwork APIを使用して指定されたメッセージを既読状態に更新します。
    /// レート制限に遭遇した場合、自動的にリトライを行います。
    ///
    /// # 引数
    ///
    /// * `room_id` - メッセージを含むルームのID。
    /// * `message_id` - 既読としてマークするメッセージのID。
    ///
    /// # 戻り値
    ///
    /// 操作が成功した場合は`Ok(())`を返します。
    /// エラーが発生した場合（APIエラー、ネットワークエラーなど）は`Error`を返します。
    ///
    /// # エラー
    ///
    /// 以下の場合にエラーを返す可能性があります：
    /// - APIがエラーレスポンスを返した場合
    /// - ネットワーク接続に問題がある場合
    /// - 最大リトライ回数を超えた場合
    ///
    /// # 例
    ///
    /// ```
    /// # use your_crate::ChatworkClient;
    /// # use your_crate::ChatworkClientTrait;
    /// # async fn example(client: &impl ChatworkClientTrait) -> Result<(), Box<dyn std::error::Error>> {
    /// let room_id = 123;
    /// let message_id = "456";
    /// client.mark_message_as_read(room_id, message_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn mark_message_as_read(
        &self,
        room_id: i32,
        message_id: &str,
    ) -> Result<ReadStatus, Error>;
}

/// Chatwork APIとの対話を管理するクライアント。
///
/// このクライアントは、Chatwork APIへのリクエストの送信、レスポンスの処理、
/// およびレート制限の処理を担当します。
pub struct ChatworkClient {
    client: Client,
    api_token: String,
}

impl ChatworkClient {
    /// 新しいChatworkClientインスタンスを作成します。
    ///
    /// # 引数
    ///
    /// * `api_token` - Chatwork APIでの認証に使用するAPIトークン。
    pub fn new(api_token: &str) -> Self {
        Self {
            client: Client::new(),
            api_token: api_token.to_string(),
        }
    }

    /// リトライロジックを使用してAPI操作を実行します。
    ///
    /// このメソッドは、レート制限が発生した場合、指数関数的バックオフを用いて
    /// 最大`MAX_RETRY_ATTEMPTS`回まで操作を再試行します。
    ///
    /// # 型パラメータ
    ///
    /// * `T` - デシリアライズされたレスポンスの型。
    /// * `F` - APIリクエストを実行するクロージャの型。
    /// * `Fut` - クロージャが返す`Future`の型。
    ///
    /// # 引数
    ///
    /// * `operation` - APIリクエストを実行するクロージャ。
    ///
    /// # 戻り値
    ///
    /// 成功した場合はデシリアライズされたレスポンスを含む`Result`、
    /// 全てのリトライが失敗した場合は`Error`を返します。
    ///
    /// # エラー
    ///
    /// 以下の場合にエラーを返します：
    /// - 全てのリトライ試行が失敗した場合
    /// - APIがエラーレスポンスを返した場合
    /// - レスポンスのデシリアライズに失敗した場合
    async fn execute_with_retry<T, F, Fut>(&self, operation: F) -> Result<T, Error>
    where
        F: Fn() -> Fut + Send + Sync,
        Fut: std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
        T: serde::de::DeserializeOwned,
    {
        let mut delay = INITIAL_RETRY_DELAY;

        for attempt in 0..MAX_RETRY_ATTEMPTS {
            if attempt > 0 {
                info!("リトライ試行 {} / {}", attempt + 1, MAX_RETRY_ATTEMPTS);
            }

            let response = operation().await?;

            if response.status().is_success() {
                return Ok(response.json().await?);
            } else if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                if attempt == MAX_RETRY_ATTEMPTS - 1 {
                    return Err(Error::MaxRetriesExceeded);
                }

                warn!(
                    "レート制限に達しました。{}秒後に再試行します...",
                    delay.as_secs()
                );
                self.log_rate_limit_headers(response.headers()).await;
                sleep(delay).await;
                delay *= 2;
            } else {
                return Err(self
                    .handle_error_response(response, "APIリクエストが失敗しました")
                    .await?);
            }
        }

        Err(Error::MaxRetriesExceeded)
    }

    /// APIレスポンスからレート制限ヘッダーをログに記録します。
    ///
    /// このメソッドは、Chatwork APIのレート制限に関する情報を抽出し、
    /// 警告レベルでログに記録します。
    ///
    /// # 引数
    ///
    /// * `headers` - レート制限情報を含むレスポンスヘッダー。
    ///
    /// # ログ出力
    ///
    /// 以下のヘッダーの値をログに記録します：
    /// - `x-ratelimit-limit`: APIリクエストの制限数
    /// - `x-ratelimit-remaining`: 残りのリクエスト数
    /// - `x-ratelimit-reset`: レート制限がリセットされる時間（UNIX時間）
    async fn log_rate_limit_headers(&self, headers: &reqwest::header::HeaderMap) {
        let limit = headers
            .get("x-ratelimit-limit")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("N/A");
        let remaining = headers
            .get("x-ratelimit-remaining")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("N/A");
        let reset = headers
            .get("x-ratelimit-reset")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("N/A");

        warn!("レート制限ヘッダー: x-ratelimit-limit: {}, x-ratelimit-remaining: {}, x-ratelimit-reset: {}", limit, remaining, reset);
    }

    /// APIからのエラーレスポンスを処理します。
    ///
    /// このメソッドは、APIからのエラーレスポンスを解析し、
    /// 詳細な情報をログに記録し、適切なエラー型を生成します。
    ///
    /// # 引数
    ///
    /// * `response` - APIからのエラーレスポンス。
    /// * `error_msg` - ログに含めるカスタムエラーメッセージ。
    ///
    /// # 戻り値
    ///
    /// APIエラーの詳細を含む`Error`オブジェクトを返します。
    ///
    /// # エラー
    ///
    /// レスポンスボディの読み取りに失敗した場合、
    /// `Error::Context`でラップされたエラーを返します。
    async fn handle_error_response(
        &self,
        response: reqwest::Response,
        error_msg: &str,
    ) -> Result<Error, Error> {
        let status = response.status();
        let headers = response.headers().clone();
        let body = response
            .text()
            .await
            .context("レスポンスボディの読み取りに失敗しました")?;
        let errors: Value = serde_json::from_str(&body).unwrap_or(Value::Null);

        error!("APIエラー: {}. ステータス: {}", error_msg, status);
        error!("レスポンスヘッダー: {:?}", headers);
        error!("レスポンスボディ: {}", body);
        error!("パースされたエラー: {:?}", errors["errors"]);

        self.log_rate_limit_headers(&headers).await;

        Ok(Error::ApiError(status, format!("{}: {:?}", error_msg, errors["errors"])).into())
    }
}

#[async_trait]
impl ChatworkClientTrait for ChatworkClient {
    async fn fetch_rooms(&self) -> Result<Vec<Room>, Error> {
        info!("ルームの取得を開始します");
        let url = "https://api.chatwork.com/v2/rooms";

        self.execute_with_retry(|| async {
            self.client
                .get(url)
                .header("X-ChatWorkToken", &self.api_token)
                .send()
                .await
        })
        .await
    }

    async fn fetch_messages(&self, room_id: i32) -> Result<Vec<Message>, Error> {
        info!("ルーム: {}のメッセージ取得を開始します", room_id);
        let url = format!("https://api.chatwork.com/v2/rooms/{}/messages", room_id);

        self.execute_with_retry(|| async {
            self.client
                .get(&url)
                .header("X-ChatWorkToken", &self.api_token)
                .send()
                .await
        })
        .await
    }

    async fn mark_message_as_read(
        &self,
        room_id: i32,
        message_id: &str,
    ) -> Result<ReadStatus, Error> {
        info!(
            "メッセージを既読としてマークします。ルーム: {}, メッセージ: {}",
            room_id, message_id
        );

        let url = format!(
            "https://api.chatwork.com/v2/rooms/{}/messages/read",
            room_id
        );

        let status: ReadStatus = self
            .execute_with_retry(|| async {
                self.client
                    .put(&url)
                    .header("X-ChatWorkToken", &self.api_token)
                    .form(&[("message_id", message_id)])
                    .send()
                    .await
            })
            .await?;

        Ok(status)
    }
}

/// `ChatworkClient`と`ChatworkClientTrait`の単体テスト。
///
/// これらのテストは`MockChatworkClientTrait`を使用して、
/// 各メソッドの期待される動作を検証します。
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use tokio;

    // MockChatworkClientTraitを使用可能にする
    use super::MockChatworkClientTrait;

    #[tokio::test]
    async fn test_fetch_rooms() {
        let mut mock_client = MockChatworkClientTrait::new();
        mock_client.expect_fetch_rooms().times(1).returning(|| {
            Ok(vec![Room {
                room_id: 1,
                unread_num: 1,
                mention_num: 0,
            }])
        });

        let rooms = mock_client.fetch_rooms().await.unwrap();
        assert_eq!(rooms.len(), 1);
        assert_eq!(rooms[0].room_id, 1);
    }

    #[tokio::test]
    async fn test_fetch_messages() {
        let mut mock_client = MockChatworkClientTrait::new();
        mock_client
            .expect_fetch_messages()
            .with(eq(123))
            .times(1)
            .returning(|_| {
                Ok(vec![Message {
                    message_id: "1".to_string(),
                    body: "テストメッセージ".to_string(),
                }])
            });

        let messages = mock_client.fetch_messages(123).await.unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].message_id, "1");
        assert_eq!(messages[0].body, "テストメッセージ");
    }

    #[tokio::test]
    async fn test_mark_message_as_read() {
        let mut mock_client = MockChatworkClientTrait::new();
        mock_client
            .expect_mark_message_as_read()
            .with(eq(123), eq("456"))
            .times(1)
            .returning(|_, _| {
                Ok(ReadStatus {
                    unread_num: 0,
                    mention_num: 0,
                })
            });

        let result = mock_client.mark_message_as_read(123, "456").await;
        assert!(result.is_ok());
        if let Ok(status) = result {
            assert_eq!(status.unread_num, 0);
            assert_eq!(status.mention_num, 0);
        }
    }

    #[tokio::test]
    async fn test_fetch_rooms_error() {
        let mut mock_client = MockChatworkClientTrait::new();
        mock_client.expect_fetch_rooms().times(1).returning(|| {
            Err(Error::ApiError(
                reqwest::StatusCode::INTERNAL_SERVER_ERROR,
                "APIエラー".to_string(),
            ))
        });

        let result = mock_client.fetch_rooms().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_fetch_messages_empty_response() {
        let mut mock_client = MockChatworkClientTrait::new();
        mock_client
            .expect_fetch_messages()
            .with(eq(123))
            .times(1)
            .returning(|_| Ok(vec![]));

        let messages = mock_client.fetch_messages(123).await.unwrap();
        assert!(messages.is_empty());
    }
}
