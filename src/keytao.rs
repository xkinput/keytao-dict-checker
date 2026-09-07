/// 编码最大长度
pub(crate) const MAX_CODE_LEN: usize = 6;

/// 是否为形码
fn is_xm(b: &&u8) -> bool {
    matches!(b, b'a' | b'i' | b'o' | b'u' | b'v')
}

/// 编码的形码长度
fn xm_len(code: &str) -> usize {
    code.as_bytes().iter().rev().take_while(is_xm).count()
}

/// 编码的音码长度
fn ym_len(code: &str) -> usize {
    code.len() - xm_len(code)
}

/// 编码的音码段
pub(crate) fn ym(code: &str) -> &[u8] {
    &code.as_bytes()[..ym_len(code)]
}

/// 所有更短合法编码
pub(crate) fn shorter(code: &str) -> impl Iterator<Item = &str> {
    (ym_len(code)..code.len()).map(|len| &code[..len])
}

/// 单字编码的构词码
pub(crate) fn stem(code: &str) -> Option<u32> {
    match code.as_bytes() {
        [a, b, c, ..] => Some(u32::from_le_bytes([*a, *b, *c, 0])),
        _ => None,
    }
}
