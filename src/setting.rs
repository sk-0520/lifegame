/// 外周の扱い
#[derive(Debug)]
pub enum Outer {
    Reverse,
    Dead,
    Live,
}

/// 設定
#[derive(Debug)]
pub struct Setting {
    pub width: u32,
    pub height: u32,
    pub outer: Outer,
}
