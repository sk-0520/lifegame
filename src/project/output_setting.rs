use serde::{Serialize, Deserialize};

/// 出力設定
#[derive(Serialize, Deserialize, Debug)]
pub struct OutputSetting {
    /// 出力ルートディレクトリ
    pub directory: String,
}
