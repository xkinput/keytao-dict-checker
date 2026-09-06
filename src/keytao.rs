/// 是否为形码
fn is_xm(b: &&u8) -> bool {
    matches!(b, b'a' | b'i' | b'o' | b'u' | b'v')
}

/// 编码的形码长度
pub(crate) fn xm_len(code: &str) -> usize {
    code.as_bytes().iter().rev().take_while(is_xm).count()
}

/// 编码的音码部分
pub(crate) fn ym(code: &str) -> &str {
    &code[..code.len() - xm_len(code)]
}

/// 单字编码的构词码
pub(crate) fn stem(code: &str) -> Option<u32> {
    match code.as_bytes() {
        [a, b, c, ..] => Some(u32::from_le_bytes([*a, *b, *c, 0])),
        _ => None,
    }
}
