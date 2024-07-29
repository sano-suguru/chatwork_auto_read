use crate::error::Error;
use config::{Config, Environment, File};
use serde::Deserialize;
use std::{collections::HashSet, env};

/// Chatworkの設定を保持する構造体です。
#[derive(Debug, Deserialize)]
pub struct ChatworkSettings {
    /// Chatwork APIのトークン
    pub api_token: String,
    /// 対象となるアカウントIDのリスト
    pub exclude_account_ids: Vec<String>,
    /// スキップするルームIDのセット（デフォルトは空）
    #[serde(default)]
    pub exclude_room_ids: HashSet<i32>,
}

/// アプリケーション全体の設定を保持する構造体です。
#[derive(Debug, Deserialize)]
pub struct Settings {
    /// Chatwork関連の設定
    pub chatwork: ChatworkSettings,
}

impl Settings {
    /// 環境変数 RUN_MODE に基づいて新しい Settings インスタンスを作成します。
    ///
    /// RUN_MODE が設定されていない場合、デフォルトで "development" を使用します。
    ///
    /// # エラー
    ///
    /// 設定ファイルの読み込みや解析に失敗した場合、`Error`を返します。
    pub fn new() -> Result<Self, Error> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        Self::new_with_mode(&run_mode)
    }

    /// 指定されたモードに基づいて新しい Settings インスタンスを作成します。
    ///
    /// # 引数
    ///
    /// * `run_mode` - 実行モード（例: "development", "production"）
    ///
    /// # エラー
    ///
    /// 設定ファイルの読み込みや解析に失敗した場合、`Error`を返します。
    fn new_with_mode(run_mode: &str) -> Result<Self, Error> {
        let s = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(Environment::with_prefix("APP"))
            .build()?;

        Ok(s.try_deserialize()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    /// テスト用の設定ファイルを作成するヘルパー関数です。
    fn create_test_config(dir: &TempDir, filename: &str, content: &str) {
        let config_path = dir.path().join(filename);
        fs::write(config_path, content).expect("テスト用設定ファイルの作成に失敗しました");
    }

    #[test]
    fn test_settings_new_with_mode() {
        let temp_dir = TempDir::new().expect("一時ディレクトリの作成に失敗しました");

        // デフォルト設定の作成
        create_test_config(
            &temp_dir,
            "config/default.toml",
            r#"
            [chatwork]
            api_token = "default_token"
            exclude_account_ids = ["123", "456"]
        "#,
        );

        // 開発環境用設定の作成
        create_test_config(
            &temp_dir,
            "config/development.toml",
            r#"
            [chatwork]
            api_token = "dev_token"
            exclude_room_ids = [1, 2, 3]
        "#,
        );

        // テスト用の設定ディレクトリを設定
        env::set_var("CONFIG_DIR", temp_dir.path());

        let settings = Settings::new_with_mode("development").expect("設定の作成に失敗しました");

        assert_eq!(settings.chatwork.api_token, "dev_token");
        assert_eq!(settings.chatwork.exclude_account_ids, vec!["123", "456"]);
        assert_eq!(settings.chatwork.exclude_room_ids, HashSet::from([1, 2, 3]));
    }

    #[test]
    fn test_settings_new_with_mode_production() {
        let temp_dir = TempDir::new().expect("一時ディレクトリの作成に失敗しました");

        // デフォルト設定の作成
        create_test_config(
            &temp_dir,
            "config/default.toml",
            r#"
            [chatwork]
            api_token = "default_token"
            exclude_account_ids = ["123", "456"]
        "#,
        );

        // 本番環境用設定の作成
        create_test_config(
            &temp_dir,
            "config/production.toml",
            r#"
            [chatwork]
            api_token = "prod_token"
        "#,
        );

        // テスト用の設定ディレクトリを設定
        env::set_var("CONFIG_DIR", temp_dir.path());

        let settings = Settings::new_with_mode("production").expect("設定の作成に失敗しました");

        assert_eq!(settings.chatwork.api_token, "prod_token");
        assert_eq!(settings.chatwork.exclude_account_ids, vec!["123", "456"]);
        assert!(settings.chatwork.exclude_room_ids.is_empty());
    }

    #[test]
    fn test_settings_new_with_env_override() {
        let temp_dir = TempDir::new().expect("一時ディレクトリの作成に失敗しました");

        // デフォルト設定の作成
        create_test_config(
            &temp_dir,
            "config/default.toml",
            r#"
            [chatwork]
            api_token = "default_token"
            exclude_account_ids = ["123", "456"]
        "#,
        );

        // テスト用の設定ディレクトリを設定
        env::set_var("CONFIG_DIR", temp_dir.path());

        // 環境変数でオーバーライド
        env::set_var("APP_CHATWORK_API_TOKEN", "env_token");

        let settings = Settings::new_with_mode("development").expect("設定の作成に失敗しました");

        assert_eq!(settings.chatwork.api_token, "env_token");
        assert_eq!(settings.chatwork.exclude_account_ids, vec!["123", "456"]);
        assert!(settings.chatwork.exclude_room_ids.is_empty());

        // クリーンアップ
        env::remove_var("APP_CHATWORK_API_TOKEN");
    }
}
