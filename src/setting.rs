/// 外周の扱い
enum Outer {
    Reverse,
    Dead,
    Live,
}

/// 設定
struct Setting {
    width: u32,
    height: u32,
    outer: Outer,
}
