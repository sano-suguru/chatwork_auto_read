/// ロギングシステムを設定します。
///
/// この関数は環境変数 `RUST_LOG` に基づいてログレベルを設定します。
/// `RUST_LOG` が設定されていない場合、デフォルトで "info" レベルを使用します。
///
/// # 使用例
///
/// ```
/// use your_crate_name::setup_logging;
///
/// fn main() {
///     setup_logging();
///     // これ以降のログ出力は設定されたレベルに従います
/// }
/// ```
///
/// # 注意
///
/// この関数は通常、アプリケーションの起動時に一度だけ呼び出されるべきです。
/// 複数回呼び出すと、予期せぬ動作を引き起こす可能性があります。
pub fn setup_logging() {
    let log_level = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();
}
