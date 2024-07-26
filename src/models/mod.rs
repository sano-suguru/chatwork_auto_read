//! Chatworkアプリケーションのデータモデル
//!
//! このモジュールは、Chatworkアプリケーションで使用される
//! 主要なデータ構造を定義します。メッセージやルームなどの
//! エンティティがここで定義され、アプリケーション全体で
//! 使用されます。

/// メッセージ関連の構造体と機能を含むモジュール
mod message;

/// ルーム関連の構造体と機能を含むモジュール
mod room;

mod read_status;

/// Chatworkのメッセージを表す構造体
///
/// この構造体は、個々のChatworkメッセージのデータを
/// 保持し、APIレスポンスのデシリアライズに使用されます。
pub use message::Message;

/// Chatworkのルームを表す構造体
///
/// この構造体は、Chatworkのルーム（チャットルーム）の
/// 情報を保持し、APIレスポンスのデシリアライズに使用されます。
pub use room::Room;

pub use read_status::ReadStatus;
