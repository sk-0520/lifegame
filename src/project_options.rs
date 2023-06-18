use serde::{Serialize, Deserialize};

/// プロジェクト設定。
#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectOptions {
    /// プロジェクトルートディレクトリ
    pub project_directory: String,
    /// 出力ルートディレクトリ
    pub output_directory: String,
}
