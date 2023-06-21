use serde::{Serialize, Deserialize};

/// プロジェクト設定
#[derive(Serialize, Deserialize, Debug)]
pub struct InputSetting {
    /// プロジェクトルートディレクトリ
    pub directory: String,
}
