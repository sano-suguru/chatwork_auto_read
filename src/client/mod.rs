//! Chatwork APIクライアントモジュール
//!
//! このモジュールは、Chatwork APIとの通信を担当するクライアントの実装と
//! 関連するトレイトを提供します。実際のAPIクライアント、モッククライアント、
//! およびクライアントの振る舞いを定義するトレイトが含まれています。

/// Chatwork APIクライアントの実装を含むサブモジュール
mod chatwork;

/// Chatwork APIと通信するための具体的なクライアント実装
pub use chatwork::ChatworkClient;

/// Chatwork APIクライアントの振る舞いを定義するトレイト
///
/// このトレイトは、実際のAPIクライアントとモッククライアントの両方で
/// 実装されます。これにより、依存性注入やテストが容易になります。
pub use chatwork::ChatworkClientTrait;

/// テスト用のモッククライアント
///
/// `mockall`クレートによって自動生成されるモッククライアントです。
/// ユニットテストやインテグレーションテストで使用されます。
#[cfg(test)]
pub use chatwork::MockChatworkClientTrait;
