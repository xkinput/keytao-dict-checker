pub(crate) const CODE_ELEMS: usize = 26;
pub(crate) const MAX_CODE_N: usize = 6;
pub(crate) const MIN_PHRASE_CODE_N: usize = 3;

/// 构词码：单字编码的前 3 个码元
pub(crate) type Stem = [u8; 3];

pub(crate) fn is_xm(c: u8) -> bool {
    matches!(c, b'a' | b'i' | b'o' | b'u' | b'v')
}

pub(crate) fn is_ym(c: u8) -> bool {
    c.is_ascii_lowercase() && !is_xm(c)
}

/// 从单字编码中提取构词码；不足时返回 `None`
pub(crate) fn stem(code: &str) -> Option<Stem> {
    match code.as_bytes() {
        [a, b, c, ..] => Some([*a, *b, *c]),
        _ => None,
    }
}

pub(crate) fn phrase_min_code_len(text: &str) -> usize {
    let mut cs = text.chars();
    if cs.nth(2).is_some() && cs.next().is_none() {
        MIN_PHRASE_CODE_N
    } else {
        4
    }
}
