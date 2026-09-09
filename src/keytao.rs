/// 构词码：单字编码的前 3 个码元
pub(crate) type Stem = [u8; 3];

/// 从单字编码中提取构词码；不足时返回 `None`
pub(crate) fn stem(code: &str) -> Option<Stem> {
    match code.as_bytes() {
        [a, b, c, ..] => Some([*a, *b, *c]),
        _ => None,
    }
}
