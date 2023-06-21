use serde::{Serialize, Deserialize};

use super::input_setting::InputSetting;
use super::output_setting::OutputSetting;

/// プロジェクト設定。
#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectOptions {
    /// 入力設定
    pub input: InputSetting,
    /// 出力設定
    pub output: OutputSetting,
}
