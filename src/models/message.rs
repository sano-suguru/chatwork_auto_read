use serde::Deserialize;

/// Chatworkのメッセージを表す構造体です。
///
/// この構造体は、Chatwork APIからのレスポンスをデシリアライズするために使用されます。
/// `serde`の`Deserialize`トレイトを実装しているため、JSONレスポンスから直接この構造体にデシリアライズできます。
#[derive(Debug, Deserialize)]
pub struct Message {
    /// メッセージの一意識別子です。
    ///
    /// この識別子は文字列形式で、Chatwork内でメッセージを一意に特定するために使用されます。
    pub message_id: String,

    /// メッセージの本文です。
    ///
    /// ここにはメッセージの実際のテキスト内容が含まれます。
    /// Chatworkの仕様に従い、メンションやリンクなどの特殊な形式も含まれる可能性があります。
    pub body: String,
}
