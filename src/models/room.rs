use serde::Deserialize;

/// Chatworkのルーム情報を表す構造体です。
///
/// この構造体は、Chatwork APIからのルーム情報レスポンスをデシリアライズするために使用されます。
/// `serde`の`Deserialize`トレイトを実装しているため、JSONレスポンスから直接この構造体にデシリアライズできます。
///
/// # フィールド
///
/// * `room_id` - ルームの一意識別子
/// * `unread_num` - 未読メッセージ数
/// * `mention_num` - メンション（呼びかけ）の数
///
/// # 使用例
///
/// ```
/// use your_crate_name::Room;
///
/// let json_data = r#"
///     {
///         "room_id": 123,
///         "unread_num": 10,
///         "mention_num": 2
///     }
/// "#;
///
/// let room: Room = serde_json::from_str(json_data).unwrap();
/// assert_eq!(room.room_id, 123);
/// ```
#[derive(Debug, Deserialize)]
pub struct Room {
    /// ルームの一意識別子です。
    ///
    /// この識別子は整数形式で、Chatwork内でルームを一意に特定するために使用されます。
    pub room_id: i32,

    /// ルーム内の未読メッセージ数です。
    ///
    /// この値は、ユーザーがまだ読んでいないメッセージの総数を示します。
    pub unread_num: i32,

    /// ルーム内のメンション（呼びかけ）の数です。
    ///
    /// この値は、ユーザーに対する未読のメンションの数を示します。
    /// メンションは通常、ユーザーの注意を特定のメッセージに向けるために使用されます。
    pub mention_num: i32,
}
