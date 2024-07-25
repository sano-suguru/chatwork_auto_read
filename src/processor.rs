use crate::client::ChatworkClientTrait;
use crate::error::Error;
use crate::models::{Message, Room};
use crate::settings::Settings;
use log::{info, warn};

/// Chatworkのメッセージを処理するための構造体です。
pub struct MessageProcessor<T: ChatworkClientTrait> {
    client: T,
    settings: Settings,
}

impl<T: ChatworkClientTrait> MessageProcessor<T> {
    /// 新しい`MessageProcessor`インスタンスを作成します。
    ///
    /// # 引数
    ///
    /// * `client` - Chatwork APIクライアントの実装
    /// * `settings` - アプリケーション設定
    pub fn new(client: T, settings: Settings) -> Self {
        Self { client, settings }
    }

    /// 全てのルームのメッセージを処理します。
    ///
    /// # エラー
    ///
    /// APIリクエストが失敗した場合、`Error`を返します。
    pub async fn process_all_rooms(&self) -> Result<(), Error> {
        let rooms = self.client.fetch_rooms().await?;
        info!("処理対象のルームが{}個見つかりました", rooms.len());

        for (index, room) in rooms.iter().enumerate() {
            info!(
                "ルームを処理中: {} / {} (ID: {})",
                index + 1,
                rooms.len(),
                room.room_id
            );
            if self.should_skip_room(room) {
                continue;
            }
            if let Err(e) = self.process_room(room).await {
                warn!("ルーム{}の処理に失敗しました: {:?}", room.room_id, e);
            } else {
                info!("ルーム{}の処理が成功しました", room.room_id);
            }
        }
        info!("全てのルームの処理が完了しました");
        Ok(())
    }

    /// 指定されたルームをスキップすべきかどうかを判断します。
    ///
    /// # 引数
    ///
    /// * `room` - 判断対象のルーム
    ///
    /// # 戻り値
    ///
    /// ルームをスキップすべき場合は`true`、そうでない場合は`false`を返します。
    fn should_skip_room(&self, room: &Room) -> bool {
        if self.settings.chatwork.skip_room_ids.contains(&room.room_id) {
            info!(
                "ルーム{}をスキップします: スキップリストに含まれています",
                room.room_id
            );
            return true;
        }
        if room.unread_num == 0 {
            info!(
                "ルーム{}をスキップします: 未読メッセージがありません",
                room.room_id
            );
            return true;
        }
        if room.mention_num > 0 {
            info!(
                "ルーム{}をスキップします: メンションが含まれています",
                room.room_id
            );
            return true;
        }
        false
    }

    /// 指定されたルームのメッセージを処理します。
    ///
    /// # 引数
    ///
    /// * `room` - 処理対象のルーム
    ///
    /// # エラー
    ///
    /// APIリクエストが失敗した場合、`Error`を返します。
    async fn process_room(&self, room: &Room) -> Result<(), Error> {
        let messages = self.client.fetch_messages(room.room_id).await?;

        if let Some(target_message) = self.find_target_message(&messages) {
            self.client
                .mark_message_as_read(room.room_id, &target_message.message_id)
                .await?;
        }

        Ok(())
    }

    /// メッセージリストから対象のメッセージを見つけます。
    ///
    /// # 引数
    ///
    /// * `messages` - 検索対象のメッセージリスト
    ///
    /// # 戻り値
    ///
    /// 対象のメッセージが見つかった場合はそのメッセージへの参照を、
    /// 見つからなかった場合は`None`を返します。
    fn find_target_message<'a>(&self, messages: &'a [Message]) -> Option<&'a Message> {
        info!(
            "{}個のメッセージから対象のメッセージを検索中",
            messages.len()
        );
        let target_mentions: Vec<String> = self
            .settings
            .chatwork
            .skip_account_ids
            .iter()
            .map(|id| format!("[To:{}]", id))
            .collect();

        let target_index = messages
            .iter()
            .position(|message| {
                target_mentions
                    .iter()
                    .any(|mention| message.body.contains(mention))
            })
            .map(|index| index.saturating_sub(1))
            .unwrap_or_else(|| messages.len().saturating_sub(1));

        let result = messages.get(target_index);
        if let Some(message) = result {
            info!(
                "対象のメッセージが見つかりました: ID {}",
                message.message_id
            );
        } else {
            warn!("対象のメッセージが見つかりませんでした");
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{client::MockChatworkClientTrait, settings::ChatworkSettings};
    use mockall::predicate::*;
    use std::collections::HashSet;

    fn create_test_settings() -> Settings {
        Settings {
            chatwork: ChatworkSettings {
                api_token: "test_token".to_string(),
                skip_account_ids: vec!["123".to_string()],
                skip_room_ids: HashSet::from([999]),
            },
        }
    }

    #[tokio::test]
    async fn test_process_all_rooms() {
        let mut mock_client = MockChatworkClientTrait::new();
        let settings = create_test_settings();

        mock_client.expect_fetch_rooms().times(1).returning(|| {
            Ok(vec![
                Room {
                    room_id: 1,
                    unread_num: 1,
                    mention_num: 0,
                },
                Room {
                    room_id: 2,
                    unread_num: 0,
                    mention_num: 0,
                },
                Room {
                    room_id: 3,
                    unread_num: 1,
                    mention_num: 1,
                },
            ])
        });

        mock_client
            .expect_fetch_messages()
            .with(eq(1))
            .times(1)
            .returning(|_| {
                Ok(vec![
                    Message {
                        message_id: "1".to_string(),
                        body: "Test message".to_string(),
                    },
                    Message {
                        message_id: "2".to_string(),
                        body: "[To:123] Test mention".to_string(),
                    },
                ])
            });

        mock_client
            .expect_mark_message_as_read()
            .with(eq(1), eq("1"))
            .times(1)
            .returning(|_, _| Ok(()));

        let processor = MessageProcessor::new(mock_client, settings);
        let result = processor.process_all_rooms().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_process_room_with_no_unread_messages() {
        let mut mock_client = MockChatworkClientTrait::new();
        let settings = create_test_settings();

        mock_client.expect_fetch_rooms().times(1).returning(|| {
            Ok(vec![Room {
                room_id: 1,
                unread_num: 0,
                mention_num: 0,
            }])
        });

        // fetch_messages と mark_message_as_read は呼ばれないはず

        let processor = MessageProcessor::new(mock_client, settings);
        let result = processor.process_all_rooms().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_process_room_with_mentions() {
        let mut mock_client = MockChatworkClientTrait::new();
        let settings = create_test_settings();

        mock_client.expect_fetch_rooms().times(1).returning(|| {
            Ok(vec![Room {
                room_id: 1,
                unread_num: 1,
                mention_num: 1,
            }])
        });

        // fetch_messages と mark_message_as_read は呼ばれないはず

        let processor = MessageProcessor::new(mock_client, settings);
        let result = processor.process_all_rooms().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_process_skipped_room() {
        let mut mock_client = MockChatworkClientTrait::new();
        let settings = create_test_settings();

        mock_client.expect_fetch_rooms().times(1).returning(|| {
            Ok(vec![Room {
                room_id: 999,
                unread_num: 1,
                mention_num: 0,
            }])
        });

        // fetch_messages と mark_message_as_read は呼ばれないはず

        let processor = MessageProcessor::new(mock_client, settings);
        let result = processor.process_all_rooms().await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_find_target_message_with_no_messages() {
        let settings = create_test_settings();
        let processor = MessageProcessor::new(MockChatworkClientTrait::new(), settings);
        let messages = vec![];
        let target_message = processor.find_target_message(&messages);
        assert!(target_message.is_none());
    }

    #[test]
    fn test_find_target_message_with_no_matching_mention() {
        let settings = create_test_settings();
        let processor = MessageProcessor::new(MockChatworkClientTrait::new(), settings);
        let messages = vec![
            Message {
                message_id: "1".to_string(),
                body: "Test message 1".to_string(),
            },
            Message {
                message_id: "2".to_string(),
                body: "Test message 2".to_string(),
            },
        ];
        let target_message = processor.find_target_message(&messages);
        assert_eq!(target_message.unwrap().message_id, "2");
    }
}
