/// 是否键道6的形码
fn is_xm(b: &&u8) -> bool {
    matches!(b, b'a' | b'i' | b'o' | b'u' | b'v')
}

/// 键道6单字编码的构词部分
pub(crate) fn stem(code: &str) -> Option<[char; 3]> {
    let mut cs = code.chars();
    match (cs.next(), cs.next(), cs.next()) {
        (Some(a), Some(b), Some(c)) => Some([a, b, c]),
        _ => None,
    }
}

/// 键道6编码末尾的形码长度
pub(crate) fn xm_len(code: &str) -> usize {
    code.as_bytes().iter().rev().take_while(is_xm).count()
}

/// 键道6编码的音码部分
pub(crate) fn ym(code: &str) -> &str {
    &code[..code.len() - xm_len(code)]
}
